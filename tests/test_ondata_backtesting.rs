use std::sync::atomic::AtomicI64;
use std::sync::mpsc::channel;
use tracing::{Event, Level, event};
use chrono::{NaiveDate, NaiveDateTime};

use rt_core::{data::{datafeed::{CSVDataFeedBuilder, DataFeed}, Resolution, data_providers::YahooFinanceTradeBar}, broker::{fill::engine::FillEngine, orders::Side}, rtengine::BackTester};
use rt_core::rtengine::RTEngine;
use rt_core::algorithm::Algo;
use rt_core::data::slice::Slice;
use rt_core::broker::{BacktestingBroker, Broker};
use rt_core::broker::slippage::simple_model::SimpleSlippageModel;
use rt_core::broker::fill::engine::BasicFillEngine;
use rt_core::SecuritySymbol;

#[test]
fn test_on_data_backtesting() {

    // Arrange
    let mut builder = CSVDataFeedBuilder::<f64, YahooFinanceTradeBar<f64>>::new(SecuritySymbol::Equity(String::from("AAPL")), Resolution::Day)
        .with_path("tests/data/AAPL_yahoo_reduced.csv");

    let my_algo = MyAlgo {};

    let start_date = NaiveDate::from_ymd_opt(2022, 4, 4).unwrap().and_hms_opt(0, 0, 0).unwrap();

    let slippage = SimpleSlippageModel::new(0.01);

    let fill_engine = BasicFillEngine::new(0.01, slippage);

    let broker = BacktestingBroker::new(0.01, fill_engine);

    let mut engine = RTEngine::builder()
        .with_mode(rt_core::rtengine::RunMode::BackTest)
        .with_algo(my_algo)
        .with_resolution(Resolution::Day)
        .with_start_time(start_date)
        .with_broker(broker)
        .build().unwrap();

    engine.set_cash(200000.0);

    engine.add_feed(builder);

    // Act

    engine.run();

    // Assert
    // assert_eq!(result, ())
}


#[derive(Clone)]
struct MyAlgo {

}

impl Algo for MyAlgo {

    type NumberType = f64;

    fn on_data<T, U>(&self, slice: Slice<Self::NumberType>, engine: &mut RTEngine<T, U>) where 
    T: Algo<NumberType = Self::NumberType>,
    U: Broker<NumberType = Self::NumberType> + BackTester {
        // event!(Level::INFO, "{:?}", slice.get_bar_by_symbol("AAPL"))
        println!("{}", slice);
        println!("{}", engine.get_time());
        if engine.get_time() == 1649376000000 {
            engine.submit_market_order(rt_core::SecuritySymbol::Equity(String::from("AAPL")), 1000.0, Side::Buy);
        }
        println!("{:?}", engine.cash_balance());
        println!("{:?}", engine.get_holding(rt_core::SecuritySymbol::Equity(String::from("AAPL"))))
        
    }
}