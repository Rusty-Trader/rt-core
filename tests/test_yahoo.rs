// use std::sync::atomic::AtomicU64;
// use std::sync::mpsc::channel;

// use rt_core::data::{datafeed::{CSVDataFeed, DataFeed}, Resolution, data_providers::YahooFinanceTradeBar};

// #[test]
// fn test_yahoo_csv_parser() {

//     // Arrange
//     let mut feed = CSVDataFeed::<f64, YahooFinanceTradeBar<f64>>::new(
//         "Test",
//         "tests/data/AAPL_yahoo.csv",
//         AtomicU64::new(1),
//         Resolution::Day
//     );

//     let (sender, receiver) = channel();

//     // Act

//     let result: () = feed.connect(sender, rt_core::rtengine::RunMode::UnitTest).unwrap();

//     // Assert
//     assert_eq!(result, ())
// }