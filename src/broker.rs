use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use std::ops::Not;

use orders::{Order, MarketOrder, FilledOrder};
use fill::engine::FillEngine;

use crate::{DataNumberType, PortfolioNumberType};
use crate::data::{DataPoint, self};
use crate::portfolio::Portfolio;
use crate::rtengine::BackTester;
use crate::time::TimeSync;

use self::error::BrokerError;
use self::fill::PortfolioData;
use self::orders::{OrderType, OrderError};
use self::slippage::SlippageModel;

pub mod orders;
pub mod error;
pub mod fill;
pub mod slippage;

pub trait Broker {

    type NumberType: DataNumberType;

    fn submit_order(&mut self, order: OrderType<Self::NumberType>);

    fn process_received_messages(&mut self);

    fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<Self::NumberType>, OrderError<Self::NumberType>>>;

    fn get_open_orders(&self) -> &HashMap<String, OrderType<Self::NumberType>>;

    fn connect(&mut self, time: TimeSync);

    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>);

    fn get_portfolio_data(&self) -> PortfolioData<Self::NumberType>;

    fn send_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>);

    // fn sync_portfolio<'a>(&mut self, portfolio: &'a Portfolio);

}



pub struct BacktestingBroker<T, U> where
    T: DataNumberType,
    U: FillEngine + BackTester {

    time: TimeSync,

    fill_engine: U,

    sender: Sender<BrokerMessage<T>>,

    receiver: Receiver<BrokerMessage<T>>,

    open_orders: HashMap<String, OrderType<T>>,

    filled_orders: Vec<Result<FilledOrder<T>, OrderError<T>>>,

    portfolio_data: Option<PortfolioData<T>>

}

impl<T, U> BacktestingBroker<T, U> where
    T: DataNumberType,
    U: FillEngine<NumberType = T> + BackTester{

    pub fn new(commission: T, mut fill_engine: U) -> Self {
        let (broker_sender, fill_receiver) = channel();
        let (fill_sender, broker_receiver) = channel();

        // TODO: Allow commission to be changed
        fill_engine.connect_to_broker(fill_sender, fill_receiver);

        Self {
            time: TimeSync::new(0, data::Resolution::Day),
            fill_engine: fill_engine,
            sender: broker_sender,
            receiver: broker_receiver,
            open_orders:HashMap::new(),
            filled_orders: Vec::new(),
            portfolio_data: None
        }
    }

    fn send_message(&mut self, mut message: BrokerMessage<T>) {
        // TODO: Add error handling
        // match &mut message {
        //     BrokerMessage::SubmitOrder(x) => {
        //         self.sender.send(message);
        //     },
        //     _ => {self.sender.send(message);},
        // }

        self.sender.send(message);
    }
    
}

impl<T, U> Broker for BacktestingBroker<T, U> where
    T: DataNumberType,
    U: FillEngine<NumberType = T> + BackTester {

    type NumberType = T;

    fn submit_order(&mut self, order: OrderType<Self::NumberType>) {
        self.open_orders.insert(String::from("Order"), order.clone());
        
        self.send_message(BrokerMessage::SubmitOrder(order))
    }

    fn process_received_messages(&mut self) {

        for msg in self.receiver.try_recv() {
            match msg {
                BrokerMessage::FilledOrder(filled) => {
                    self.filled_orders.push(filled.clone());
                    match filled {
                        Ok(x) => {self.open_orders.remove(x.get_id().as_str());},
                        Err(e) => {self.open_orders.remove(e.get_id().as_str());}
                    }
                    
                },
                BrokerMessage::PortfolioInfo(x) => {
                    self.portfolio_data = Some(x);
                }
                _ => (),
                // BrokerMessage::OrderError(err) => {
                    
                // }
            }
        }
    }

    // TODO: return output as reference
    fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<T>, OrderError<T>>> {
        // let tmp = self.filled_orders.clone();
        // self.filled_orders.clear();
        // tmp

        let tmp = self.filled_orders.clone();
        self.filled_orders.clear();
        tmp

    }

    fn get_open_orders(&self) -> &HashMap<String, OrderType<Self::NumberType>> {
        &self.open_orders
    }

    fn connect(&mut self, time: TimeSync) {
        self.time = time.clone();
        self.fill_engine.connect_to_engine(time);
    }

    // TODO: Add check to ensure broker is connected
    fn connect_to_data(&mut self, data_receiver: Receiver<DataPoint<Self::NumberType>>) {
        self.fill_engine.connect_to_data(data_receiver);
    }

    fn get_portfolio_data(&self) -> PortfolioData<Self::NumberType> {
        // TODO: Remove unwrap
        self.portfolio_data.as_ref().unwrap().clone()
    }

    fn send_portfolio_data(&mut self, data: PortfolioData<Self::NumberType>) {
        self.fill_engine.update_portfolio_data(data)
    }


}

impl<T, U> BackTester for BacktestingBroker<T, U> where
    T: DataNumberType,
    U: FillEngine<NumberType = T> + BackTester {

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

    PortfolioInfo(PortfolioData<T>),
    // OrderError(OrderError<T>)

}