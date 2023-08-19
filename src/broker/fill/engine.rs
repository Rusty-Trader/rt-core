use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::ops::{Mul, Add, Sub};

use super::super::{Order, BrokerMessage};
use super::super::orders::Side;
use super::super::slippage::SlippageModel;
use super::super::error::BrokerError;
use super::PortfolioData;

use crate::portfolio::Portfolio;
use crate::{DataNumberType, Security, PortfolioNumberType};
use crate::broker::orders::{FilledOrder, OrderType, MarketOrder, OrderError};
use crate::data::DataType;
use crate::time::TimeSync;
use crate::{data::DataPoint, rtengine::BackTester};
use crate::utils::Merge;




pub trait FillEngine {

    type NumberType: DataNumberType;

    type SlippageType: SlippageModel;

    // fn is_matched(&mut self, order: &Box<dyn Order>) -> bool;

    fn new(comission: Self::NumberType, slippage: Self::SlippageType) -> Self;

    fn connect_to_broker(&mut self, sender: Sender<BrokerMessage<Self::NumberType>>, receiver: Receiver<BrokerMessage<Self::NumberType>>);

    fn connect_to_engine(&mut self, time: TimeSync);

    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>);

    fn check_fill(&self, order: OrderType<Self::NumberType>) -> Option<Result<FilledOrder<Self::NumberType>, OrderError<Self::NumberType>>> {
        match order {
            OrderType::MarketOrder(x) => self.fill_market_order(x)
        }
    }

    fn check_funds(&self, order: MarketOrder<Self::NumberType>, price: Self::NumberType) -> Result<(), OrderError<Self::NumberType>>;

    fn fill_market_order(&self, order: MarketOrder<Self::NumberType>) -> Option<Result<FilledOrder<Self::NumberType>, OrderError<Self::NumberType>>>;
    
    fn add_datapoint(&mut self, datapoint: DataPoint<Self::NumberType>);

    fn get_commission(&self, order: OrderType<Self::NumberType>) -> Self::NumberType;

    fn process_received_messages(&mut self);

    fn update_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>);

    // fn with_portfolio<'a>(&'a mut self, portfolio: &'a Portfolio<Self::PortfolioNumberType, Self::NumberType>);

    // fn get_best_ask_price(&self, datapoint: &DataPoint<Self::NumberType>) -> Self::NumberType;

    // fn get_best_bid_price(&self, datapoint: &DataPoint<Self::NumberType>) -> Self::NumberType;

}


pub struct BasicFillEngine<T, U> where 
    T: DataNumberType,
    U: SlippageModel {

    time: TimeSync,

    data_receiver: Option<Receiver<DataPoint<T>>>,

    data_lines: HashMap<Security, DataPoint<T>>,

    open_orders: HashMap<String, OrderType<T>>,

    sender: Option<Sender<BrokerMessage<T>>>,

    receiver: Option<Receiver<BrokerMessage<T>>>,

    slippage: U,

    commission: T,

    portfolio: Option<PortfolioData<T>>

}

impl<T, U> BasicFillEngine<T, U>
    where T: DataNumberType,
    U: SlippageModel {


}

impl<T, U> BackTester for BasicFillEngine<T, U> 
    where T: DataNumberType,
    U: SlippageModel<NumberType = T> {

    fn next_cycle(&mut self) -> Result<(), crate::error::Error> {

        let mut tmp = Vec::new();

        match &self.data_receiver {
            Some(receiver) => {

                for point in receiver.try_iter() {
                    tmp.push(point);
                }
            },
            None => (),
        }

        for point in tmp {
            self.add_datapoint(point);
        }
            // .collect::<Option<Vec<DataPoint<T>>>>();

        if let Some(receiver) = &self.receiver {
            for message in receiver.try_iter() {
                match message {
                    BrokerMessage::SubmitOrder(x) => {
                        self.open_orders.insert(x.get_id(), x);
                    }
                    _ => (),
                }
            }
        }


        let mut remove = Vec::new();

        for (id, order) in &self.open_orders {

            if let Some(x) = &self.check_fill(order.clone()) {
                let _ = match &self.sender {
                    Some(sender) => sender.send(BrokerMessage::FilledOrder(x.clone())).map_err(|_| BrokerError::FillEngineError(format!("Sender error"))),
                    None => Err(BrokerError::FillEngineError(format!("Fill ending must be connected")))?,
                };
                remove.push(id.clone())
            }

            // match &self.check_fill(order.clone()) {

            //     Some()
                // Ok(y) => {
                //     if let Some(x) =  y {
                //         // TODO: Add error
                //         let _ = match &self.sender {
                //             Some(sender) => sender.send(BrokerMessage::FilledOrder(Ok(x.clone()))).map_err(|_| BrokerError::FillEngineError(format!("Sender error"))),
                //             None => Err(BrokerError::FillEngineError(format!("Fill ending must be connected")))?,
                //         };
                //         remove.push(id.clone())
                //     }
                // },
                // Err(e) => {
                //     let _ = match &self.sender {
                //         Some(sender) => sender.send(BrokerMessage::FilledOrder(Err(e.clone()))).map_err(|_| BrokerError::FillEngineError(format!("Sender error"))),
                //         None => Err(BrokerError::FillEngineError(format!("Fill ending must be connected")))?,
                //     };
                //     remove.push(id.clone())
                // }

        }

        self.open_orders.retain(|f, _| !remove.contains(f));

        Ok(())
    }
}


impl<T, U> FillEngine for BasicFillEngine<T, U> where 
    T: DataNumberType,
    U: SlippageModel<NumberType = T> {

    type NumberType = T;

    type SlippageType = U; 

    fn new(commission: Self::NumberType, slippage: Self::SlippageType) -> Self {
        Self { 
            time: TimeSync::new(0, crate::data::Resolution::Day),
            data_receiver: None,
            data_lines: HashMap::new(),
            open_orders: HashMap::new(),
            sender: None,
            receiver: None,
            slippage,
            commission,
            portfolio: None
        }
    }

    fn connect_to_broker(&mut self, sender: Sender<BrokerMessage<Self::NumberType>>, receiver: Receiver<BrokerMessage<Self::NumberType>>) {
        self.sender = Some(sender);
        self.receiver = Some(receiver);
    }

    fn connect_to_engine(&mut self, time: TimeSync) {
        self.time = time;
    }

    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>) {
        self.data_receiver = Some(data_receiver);
    }

    fn check_funds(&self, order: MarketOrder<Self::NumberType>, price: Self::NumberType) -> Result<(), OrderError<Self::NumberType>> {
        // TODO: Add ability for account margin
        // Check account has enough money
        if let Some(portfolio) = &self.portfolio {
            match order.get_side() {
                Side::Buy => {
                    if portfolio.get_cash() < (order.get_volume() * price) {
                        return Err(OrderError::new(OrderType::MarketOrder(order), self.time.get_time(), "Insufficient Funds"))
                    }
                },
                Side::Sell => {
                    match portfolio.get_holding_amount(order.get_symbol()) {
                        Some(amnt) => {
                            if amnt < &order.get_volume() {
                                return Err(OrderError::new(OrderType::MarketOrder(order), self.time.get_time(), "Insufficient Holdings"))
                            }
                        },
                        None => {
                            return Err(OrderError::new(OrderType::MarketOrder(order), self.time.get_time(), "No Holdings"))
                        }
                    }
                }
            }
        } else {
            panic!("No portfolio data available to fill engine")
        }
        Ok(())
        //Err(OrderError::new(OrderType::MarketOrder(order), self.time.get_time(), "No Portfolio added to engine"))
    }

    fn fill_market_order(&self, order: MarketOrder<Self::NumberType>) -> Option<Result<FilledOrder<Self::NumberType>, OrderError<Self::NumberType>>> {
        
        let last_data = self.data_lines.get(&order.get_symbol());

        if let Some(x) = last_data {

            if x.get_time() > order.get_timestamp() {

                let price = self.slippage.get_slippage_approximation(x, order.clone());

                if let Err(e) = self.check_funds(order.clone(), price) {
                    return Some(Err(e))
                }
                
                // TODO: Remove match
                match order.get_side() {
                    Side::Buy => {
                        return Some(Ok(FilledOrder::new(
                            OrderType::MarketOrder(order.clone()),
                            self.time.get_time(),
                            order.get_volume(),
                            price,
                            self.get_commission(OrderType::MarketOrder(order.clone())),
                            false
                        )))

                    },
                    Side::Sell => {
                        return Some(Ok(FilledOrder::new(
                            OrderType::MarketOrder(order.clone()),
                            self.time.get_time(),
                            order.get_volume(),
                            price,
                            self.get_commission(OrderType::MarketOrder(order.clone())),
                            false
                        )))
                    },
                }
            }
        }

        None

    }



    fn get_commission(&self, order: OrderType<Self::NumberType>) -> Self::NumberType {
        self.commission.clone()
    }


    // }

    fn add_datapoint(&mut self, datapoint: DataPoint<T>) {
        if let Some(x) = self.data_lines.get_mut(&datapoint.get_symbol()) {
            *x = datapoint
        } else {
            self.data_lines.insert(datapoint.get_symbol(), datapoint);
        }

    }

    fn process_received_messages(&mut self) {
        
        if let Some(receiver) = &self.receiver {
            for msg in receiver.try_recv() {
                match msg {
                    BrokerMessage::PortfolioInfo(x) => {
                        self.portfolio.as_mut().map(|y| y.merge(x));
                    },
                    _ => {}
                }
            }

        }

    }

    fn update_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>) {

        match &mut self.portfolio {
            Some(x) => {
                self.portfolio = Some({x.merge(data); x.clone()})
            }
            None => {
                self. portfolio = Some(data)
            }
        };
    }

    // fn with_portfolio<'b>(&'b mut self, portfolio: &'b Portfolio<Self::PortfolioNumberType, Self::NumberType>) {
    //     self.portfolio = Some(portfolio);
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::mpsc::channel;

    use crate::{broker::slippage::simple_model::SimpleSlippageModel, data::{tradebars::TradeBar, Resolution}};
    use crate::test_utils::setup_data_line_daily;
    
    #[test]
    fn test_basic_fill_engine_check_fill() {

        // Arrange
        let slippage_model = SimpleSlippageModel::new(0.01);

        let mut basic_fill_engine = BasicFillEngine::new(0.01, slippage_model);

        let mut data_line = setup_data_line_daily();

        basic_fill_engine.add_datapoint(data_line.pop().unwrap());

        basic_fill_engine.connect_to_engine(TimeSync::new(1000, Resolution::Day));

        let mut order = OrderType::MarketOrder(MarketOrder::new("1", Security::Equity(String::from("AAPL")), 1649289600000, 1000.0, Side::Buy));

        // order.set_timestamp(1649289600000);

        let expected = FilledOrder::new(
            order.clone(),
            1000,
            1000.0,
            171.160004 + 0.01 * 171.160004,
            0.01,
            false
        );

        // Act
        let result = basic_fill_engine.check_fill(order).unwrap();

        // Assert
        assert_eq!(result.unwrap(), expected)
        
    }
}

