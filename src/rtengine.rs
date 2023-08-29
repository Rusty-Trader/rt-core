
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use chrono::NaiveDateTime;

use crate::security::{Security, Equity};
use crate::{SecuritySymbol, data};
use crate::algorithm::Algo;
use crate::broker::fill::PortfolioData;
use crate::broker::fill::engine::FillEngine;
use crate::data::{datafeed, Resolution};
use crate::data::datamanger::DataManager;
use crate::data::datafeed::DataFeed;
use crate::error::Error;
use crate::data::datafeed::DataFeedBuilder;
use crate::portfolio::{Portfolio, Holding};
use crate::time::TimeSync;
use crate::broker::{Broker, BacktestingBroker};
use crate::broker::orders::{Order, Side, MarketOrder, FilledOrder, OrderError};
use crate::security::Currency;

pub struct RTEngine<T, U> where U: Broker + BackTester {

    data_manager: DataManager<f64>,

    portfolio: Portfolio<f64, f64>,

    broker: U,

    algo: T,

    time: TimeSync,

    mode: RunMode

}

impl<T, U> RTEngine<T, U> where
    T: Algo<NumberType = f64>,
    U: Broker<NumberType = f64> + BackTester
    {


    pub fn run(&mut self) {

        self.initialize();

        match self.mode {
            RunMode::LiveTrade => (),
            RunMode::PaperTrade => (),
            RunMode::BackTest => self.run_backtest(),
            RunMode::UnitTest => (),
        }

    }

    fn initialize(&mut self) {

        self.connect_feeds();

        // self.broker.connect(self.time.clone());

    }

    fn run_backtest(&mut self) {


        let algo = self.algo.clone();
        
        while !self.data_manager.is_finished() {

            // Update Data
            self.data_manager.feeds_send_backtest().unwrap(); // TODO: Process error

            // Get Slice and update data to Broker
            let slice = self.data_manager.get_slice();

            // Get latest portfolio details to fill engine
            self.broker.send_portfolio_data(PortfolioData{cash: self.portfolio.get_cash(), holdings: HashMap::new()});

            // Process Fill Orders
            self.broker.next_cycle().unwrap();

            // Update portfolio information
            self.update_holdings();

            // Pass Slice to Algorithm

            if slice.has_data() {
                algo.on_data(slice, self)
            }

            // Update Time
            self.time.update_time(self.mode);

        }
    }

    fn connect_feeds(&mut self) {
        // TODO: Add error
        self.data_manager.connect().unwrap()
    }



    pub fn builder() -> EngineBuilder<T, U> {
        EngineBuilder::new()
    }

    pub fn add_equity(&mut self, symbol: &str) {
        // TODO: Currently there is only support for US Stocks - add multi currency support.
        self.add_security(
            SecuritySymbol::Equity(symbol.to_owned()),
            Security::Equity(
                Equity::new(
                    Currency::USD
                )
            )
        )
    }

    pub fn add_security(&mut self, symbol: SecuritySymbol, details: Security) {
        self.portfolio.register_security(symbol, details)
    }

    pub fn add_feed<D: DataFeedBuilder<NumberType = f64>>(&mut self, datafeed_builder: D)
        where <D as DataFeedBuilder>::Output: 'static + DataFeed<NumberType = f64> {
        self.data_manager.add_feed(datafeed_builder)
    }

    pub fn get_time(&self) -> i64 {
        self.time.get_time()
    }

    pub fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<f64>, OrderError<f64>>> {
        self.portfolio.get_filled_orders().values().cloned().collect()
    }

    pub fn submit_market_order(&mut self, symbol: SecuritySymbol, volume: f64, side: Side) -> Result<(), Error> {

        Ok(self.broker.submit_order(
            crate::broker::orders::OrderType::MarketOrder(
                MarketOrder::new("Order", symbol, self.time.get_time(), volume, side)
            )
        ))

    }

    pub fn set_cash(&mut self, cash: f64) {
        self.portfolio.set_cash(cash.into())
    }

    pub fn cash_balance(&self) -> f64 {
        self.portfolio.get_cash()
    }

    pub fn get_holding(&self, symbol: SecuritySymbol) -> Option<&Holding<f64>> {
        self.portfolio.get_holding(symbol)
    }

    fn update_holdings(&mut self) {
        self.portfolio.update_holdings(
            self.broker.get_filled_orders()
        )
    }

}



pub struct EngineBuilder<T, U> where
    U: Broker {

    algo: Option<T>,

    time: Arc<AtomicI64>,

    mode: Option<RunMode>,

    resolution: Option<Resolution>,

    broker: Option<U>

}

impl<T, U> EngineBuilder<T, U> where
    T: Algo,
    U: Broker<NumberType = f64> {

    pub fn new() -> Self {
        Self {
            algo: None,
            time: Arc::new(AtomicI64::new(0)),
            mode: None,
            resolution: None,
            broker: None
        }
    }

    pub fn build(self) -> Result<RTEngine<T, U>, Error> where
        U: Broker + BackTester {

        let time_sync: TimeSync =  TimeSync::from_atomic(self.time, self.resolution
            .ok_or(Error::IncompleteBuilder(format!("Must set the minimum resolution of the data")))?);

        let mut data_manager = DataManager::new(time_sync.clone(), self.mode
            .ok_or(Error::IncompleteBuilder(format!("Engine must have a Run Mode")))?);
            // .ok_or(Error::IncompleteBuilder(format!("Engine must have a Run Mode")))?

        // &mut self.broker.map(|x| x.connect_to_data(data_manager.with_fill_sender()));

        let mut broker = self.broker.ok_or(Error::IncompleteBuilder(format!("Broker must be specified")))?;

        broker.connect_to_data(data_manager.with_fill_sender());

        broker.connect(time_sync.clone());

        Ok(RTEngine {
            data_manager: data_manager,
            portfolio: Portfolio::new(),
            broker: broker,
            algo: self.algo
                .ok_or(Error::IncompleteBuilder(format!("Algorithm must be added before engine can be built")))?,
            time: time_sync,
            mode: self.mode
                .ok_or(Error::IncompleteBuilder(format!("Engine must have a run mode")))?
        })
    }


    // TODO: have start time as date format rather than epoch
    pub fn with_start_time(self, date: NaiveDateTime) -> Self {
        Self {
            time: Arc::new(AtomicI64::new(date.timestamp_millis() as i64)),
            ..self
        }
    }

    pub fn with_start_time_unix(self, time: i64) -> Self {
        Self {
            time: Arc::new(AtomicI64::new(time)),
            ..self
        }
    }

    pub fn with_mode(self, mode: RunMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    pub fn with_algo(self, algo: T) -> Self {
        Self {
            algo: Some(algo),
            ..self
        }
    }

    pub fn with_resolution(self, resolution: Resolution) -> Self {
        Self {
            resolution: Some(resolution),
            ..self
        }
    }

    pub fn with_broker(self, broker: U) -> Self {
        Self {
            broker: Some(broker),
            ..self
        }
    }


}




pub trait BackTester {
    fn next_cycle(&mut self) -> Result<(), Error>;
}

#[derive(Copy, Clone)]
pub enum RunMode {
    LiveTrade,
    PaperTrade,
    BackTest,
    UnitTest
}