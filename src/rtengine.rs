use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use chrono::NaiveDateTime;

use crate::security::{Currency, Security, SecuritySymbol};
use crate::algorithm::Algo;
use crate::broker::fill::engine::FillEngine;
use crate::data::{DataSymbolProperties, deserialize_symbol_properties, Resolution};
use crate::data::datamanger::DataManager;
use crate::data::datafeed::DataFeed;
use crate::error::Error;
use crate::data::datafeed::DataFeedBuilder;
use crate::portfolio::{Holding, Portfolio};
use crate::time::TimeSync;
use crate::broker::Broker;
use crate::broker::orders::{FilledOrder, MarketOrder, OrderError, Side};

pub struct RTEngine<T, U> where U: Broker + BackTester {

    data_manager: DataManager<f64>,

    portfolio: Rc<RefCell<Portfolio<f64, f64>>>,

    broker: U,

    algo: T,

    time: TimeSync,

    mode: RunMode

}

impl<T, U> RTEngine<T, U> where
    T: Algo<NumberType = f64>,
    U: Broker<NumberType = f64, PortfolioNumberType = f64> + BackTester
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
            // self.broker.send_portfolio_data(PortfolioData{cash: self.portfolio.borrow_mut().get_cash(), holdings: HashMap::new()});

            // Process Fill Orders
            self.broker.next_cycle().unwrap();

            // Update portfolio information
            // self.update_holdings();

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

    // pub fn add_equity(&mut self, symbol: &str) {
    //     // TODO: Currently there is only support for US Stocks - add multi currency support.
    //     self.add_security(
    //         SecuritySymbol::Equity(symbol.to_owned()),
    //         Security::Equity(
    //             Equity::new(
    //                 Currency::USD
    //             )
    //         )
    //     )
    // }

    pub fn register_security(&mut self, security: SecuritySymbol, market: &str) {

        let result = deserialize_symbol_properties();

        match result {
            Ok(symbols) => {
                match symbols.clone().into_iter().filter(|r| r.symbol == security.symbol()).collect::<Vec<_>>().first() {
                    Some(properties) => {
                        self.register_custom_security(security, properties.clone().to_security())
                    },
                    None => {
                        match symbols.into_iter().filter(|r| (r.symbol == "*") &
                            (r.market == market) & (r.security_type == security.security_type())).collect::<Vec<DataSymbolProperties>>().first() {
                            Some(properties) => {
                                self.register_custom_security(security, properties.clone().to_security())
                            }
                            None => {
                                panic!("Could not register {}", security.symbol())
                            }
                        }
                    }
                }
            },
            Err(_) => {
                panic!("Could not load security database file")
            }
        }
    }

    pub fn register_custom_security(&mut self, symbol: SecuritySymbol, details: Security) {
        self.portfolio.borrow_mut().register_security(symbol, details)
    }

    pub fn add_feed<D: DataFeedBuilder<NumberType = f64>>(&mut self, datafeed_builder: D)
        where <D as DataFeedBuilder>::Output: 'static + DataFeed<NumberType = f64> {

        // Register securities from data feeds with the Portfolio
        for (symbol, market) in datafeed_builder.get_symbols() {
            if !self.portfolio.borrow().is_registered(symbol.clone()) {
                self.register_security(symbol.clone(), market)
            }
        }

        self.data_manager.add_feed(datafeed_builder)

    }

    pub fn get_time(&self) -> i64 {
        self.time.get_time()
    }

    pub fn get_filled_orders(&mut self) -> Vec<Result<FilledOrder<f64>, OrderError<f64>>> {
        self.portfolio.borrow_mut().get_filled_orders().values().cloned().collect()
    }

    pub fn submit_market_order(&mut self, symbol: SecuritySymbol, volume: f64, side: Side) -> Result<(), Error> {
        Ok(self.broker.submit_order(
            crate::broker::orders::OrderType::MarketOrder(
                MarketOrder::new("Order", symbol, self.time.get_time(), volume, side)
            )
        ))
    }

    fn adjust_price_for_min_price_var(&self, symbol: &SecuritySymbol, price: f64) -> f64 {

        let min_var = self.portfolio.borrow().security_details(symbol).unwrap().get_minimum_price_variation(); // TODO: error handle

        round_down(price, min_var)
    }

    pub fn set_cash(&mut self, currency: Currency, cash: f64) {
        self.portfolio.borrow_mut().set_cash(currency, cash.into())
    }

    pub fn cash_balance(&self, currency: Currency) -> Option<f64> {
        self.portfolio.borrow_mut().get_cash(currency).map(|x| x.to_owned())
    }

    pub fn get_holding(&self, symbol: SecuritySymbol) -> Option<Holding<f64>> {
        self.portfolio.borrow_mut().get_holding(symbol)
    }

    // fn update_holdings(&mut self) {
    //     self.portfolio.borrow_mut().update_holdings(
    //         self.broker.get_filled_orders()
    //     )
    // }

}

fn round_down(x: f64, a: f64) -> f64 {
    (x/a).floor() * a
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
    U: Broker<NumberType = f64, PortfolioNumberType = f64> {

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

        let portfolio: Rc<RefCell<Portfolio<f64, f64>>> = Rc::new(RefCell::new(Portfolio::new(Currency::USD)));

        // &mut self.broker.map(|x| x.connect_to_data(data_manager.with_fill_sender()));

        let mut broker = self.broker.ok_or(Error::IncompleteBuilder(format!("Broker must be specified")))?;

        broker.connect_to_data(data_manager.with_fill_sender());

        broker.connect(time_sync.clone(), portfolio.clone());

        Ok(RTEngine {
            data_manager: data_manager,
            portfolio: portfolio,
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


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::broker::BacktestingBroker;
    use crate::broker::fill::engine::BasicFillEngine;
    use crate::broker::slippage::simple_model::SimpleSlippageModel;
    use super::*;

    use crate::rtengine::RTEngine;
    use crate::data::datamanger::DataManager;
    use crate::data::slice::Slice;
    use crate::security::{Currency, Equity};

    #[derive(Clone)]
    struct TestAlgo {}

    impl Algo for TestAlgo {
        
        type NumberType = f64;
        fn on_data<T, U>(&self, slice: Slice<Self::NumberType>, engine: &mut RTEngine<T, U>) where
            T: Algo<NumberType=Self::NumberType>,
            U: Broker<NumberType=Self::NumberType, PortfolioNumberType=Self::NumberType> + BackTester {
        }
    }

    #[test]
    fn test_register_security_equity() {

        // Arrange
        let start_date = NaiveDate::from_ymd_opt(2022, 4, 4).unwrap().and_hms_opt(0, 0, 0).unwrap();

        let algo = TestAlgo {};

        let slippage = SimpleSlippageModel::new(0.01);

        let fill_engine = BasicFillEngine::new(0.01, slippage);

        let broker = BacktestingBroker::new(0.01, fill_engine);

        let mut engine = RTEngine::builder()
            .with_mode(crate::rtengine::RunMode::UnitTest)
            .with_algo(algo)
            .with_resolution(Resolution::Day)
            .with_start_time(start_date)
            .with_broker(broker)
            .build().unwrap();

        let expected = &Security::Equity(Equity::new(Currency::USD, 0.01));
        // Act

        engine.register_security(SecuritySymbol::Equity(String::from("AAPL")), "usa");

        let binding = engine.portfolio.borrow();
        let result = binding.security_details(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap();

        // Assert
        assert_eq!(result, expected)
    }
}