use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;

use orders::{Order, FilledOrder};
use fill::engine::FillEngine;

use crate::{DataNumberType, PortfolioNumberType};
use crate::data::{DataPoint, self};
use crate::portfolio::Portfolio;
use crate::rtengine::BackTester;
use crate::time::TimeSync;

use self::orders::{OrderType, OrderError};

pub mod orders;
pub mod error;
pub mod fill;
pub mod slippage;

pub trait Broker {

    type NumberType: DataNumberType;

    type PortfolioNumberType: PortfolioNumberType;

    fn submit_order(&mut self, order: OrderType<Self::NumberType>);

    fn process_received_messages(&mut self);

    // fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<Self::NumberType>, OrderError<Self::NumberType>>>;

    fn get_open_orders(&self) -> &HashMap<String, OrderType<Self::NumberType>>;

    fn connect(&mut self, time: TimeSync, portfolio: Rc<RefCell<Portfolio<Self::PortfolioNumberType, Self::NumberType>>>);

    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>);

    // fn get_portfolio_data(&self) -> PortfolioData<Self::NumberType>;

    // fn send_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>);

    // fn sync_portfolio<'a>(&mut self, portfolio: &'a Portfolio);

}



pub struct BacktestingBroker<T, U, F> where
    T: DataNumberType,
    U: FillEngine + BackTester,
    F : PortfolioNumberType {

    time: TimeSync,

    fill_engine: U,

    // sender: Sender<BrokerMessage<T>>,

    // receiver: Receiver<BrokerMessage<T>>,

    open_orders: HashMap<String, OrderType<T>>,

    portfolio: Option<Rc<RefCell<Portfolio<F, T>>>>

}

impl<T, U, F> BacktestingBroker<T, U, F> where
    T: DataNumberType,
    U: FillEngine<NumberType = T> + BackTester,
    F: PortfolioNumberType {

    pub fn new(commission: T, mut fill_engine: U) -> Self {
        // let (broker_sender, fill_receiver) = channel();
        // let (fill_sender, broker_receiver) = channel();

        // TODO: Allow commission to be changed
        // fill_engine.connect_to_broker(fill_sender, fill_receiver);

        Self {
            time: TimeSync::new(0, data::Resolution::Day),
            fill_engine: fill_engine,
            // sender: broker_sender,
            // receiver: broker_receiver,
            open_orders:HashMap::new(),
            portfolio: None
        }
    }

    fn send_message(&mut self, message: BrokerMessage<T>) {
        // TODO: Add error handling
        // match &mut message {
        //     BrokerMessage::SubmitOrder(x) => {
        //         self.sender.send(message);
        //     },
        //     _ => {self.sender.send(message);},
        // }
        // self.sender.send(message);
        self.fill_engine.add_message(message)
    }
    
}

impl<T, U, F> Broker for BacktestingBroker<T, U, F> where
    T: DataNumberType + Into<F>,
    U: FillEngine<NumberType = T, PortfolioNumberType = F> + BackTester,
    F: PortfolioNumberType {

    type NumberType = T;

    type PortfolioNumberType = F;

    fn submit_order(&mut self, order: OrderType<Self::NumberType>) {
        self.open_orders.insert(String::from("Order"), order.clone());
        
        self.send_message(BrokerMessage::SubmitOrder(order))
    }

    fn process_received_messages(&mut self) {

        for msg in self.fill_engine.get_filled_orders() {
            match msg {
                BrokerMessage::FilledOrder(filled) => {
                    // self.filled_orders.push(filled.clone());
                    match &self.portfolio {
                        Some(portfolio) => {
                            portfolio.borrow_mut().update_holding(filled.clone());
                        },
                        None => panic!("Portfolio must be connect to broker")
                    }

                    match filled {
                        Ok(x) => {self.open_orders.remove(x.get_id().as_str());},
                        Err(e) => {self.open_orders.remove(e.get_id().as_str());}
                    }
                    
                },
                // BrokerMessage::PortfolioInfo(x) => {
                //     self.portfolio = Some(x);
                // }
                _ => (),
                // BrokerMessage::OrderError(err) => {
                    
                // }
            }
        }
    }

    // TODO: return output as reference
    // fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<T>, OrderError<T>>> {
    //     // let tmp = self.filled_orders.clone();
    //     // self.filled_orders.clear();
    //     // tmp

    //     let tmp = self.filled_orders.clone();
    //     self.filled_orders.clear();
    //     tmp

    // }

    fn get_open_orders(&self) -> &HashMap<String, OrderType<Self::NumberType>> {
        &self.open_orders
    }

    fn connect(&mut self, time: TimeSync, portfolio: Rc<RefCell<Portfolio<F, T>>>) {
        self.time = time.clone();
        self.fill_engine.connect_to_engine(time, portfolio.clone());
        self.portfolio = Some(portfolio)
    }

    // TODO: Add check to ensure broker is connected
    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>) {
        self.fill_engine.connect_to_data(data_receiver);
    }

    // fn get_portfolio_data(&self) -> PortfolioData<Self::NumberType> {
    //     // TODO: Remove unwrap
    //     self.portfolio.as_ref().unwrap().clone()
    // }

    // fn send_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>) {
    //     self.fill_engine.update_portfolio_data(data)
    // }


}

impl<T, U, F> BackTester for BacktestingBroker<T, U, F> where
    T: DataNumberType + Into<F>,
    U: FillEngine<NumberType = T, PortfolioNumberType = F> + BackTester,
    F: PortfolioNumberType {

    fn next_cycle(&mut self) -> Result<(), crate::error::Error> {

        self.fill_engine.next_cycle()?;

        self.process_received_messages();


        // TODO: Finish
        // self.fill_engine.check_fill();

        Ok(())

    }

}


#[derive(Debug)]
pub enum BrokerMessage<T> where T: DataNumberType {

    SubmitOrder(OrderType<T>),

    FilledOrder(Result<FilledOrder<T>, OrderError<T>>),

    // PortfolioInfo(PortfolioData<T>),
    // OrderError(OrderError<T>)

}