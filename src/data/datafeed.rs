use std::collections::HashMap;
use std::fmt;
use std::error;
use csv::{Reader};
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, Receiver, SendError};
use tracing::{event, Level};

use super::DataPoint;
use super::DataType;
use super::Resolution;
use super::data_providers::IntoDataPoint;
use super::tradebars::TradeBar;
use super::error::DataError;

use crate::security::SecuritySymbol;
use crate::rtengine::RunMode;


pub trait DataFeed {

    type NumberType: Clone;

    fn get_symbols(&self) -> &Vec<(SecuritySymbol, String)>;
    fn connect(&mut self, sender: Sender<DataPoint<Self::NumberType>>, mode: RunMode) -> Result<(), DataError>;
    fn send_backtest(&mut self) -> Result<(), DataError>;
    fn is_finished(&self) -> bool;
}

pub trait DataFeedBuilder {

    type Output;

    type NumberType: Clone;

    fn build(self) -> Result<Self::Output, DataError>;

    fn with_mode(self, mode: RunMode) -> Self;

    fn with_time(self, time: Arc<AtomicI64>) -> Self;

    fn with_sender(self, sender: Sender<DataPoint<Self::NumberType>>) -> Self;

    fn with_fill_sender(self, sender: Sender<DataPoint<Self::NumberType>>) -> Self;

    fn get_symbols(&self) -> &Vec<(SecuritySymbol, String)>;
}

pub struct CSVDataFeed<T, U> where T: Clone {
    path: PathBuf,
    symbols: Vec<(SecuritySymbol, String)>,
    data: Vec<DataPoint<T>>,
    time: Arc<AtomicI64>,
    resolution: Resolution,
    sender: Sender<DataPoint<T>>,
    mode: RunMode,
    phantom: PhantomData<U>
}


impl<T, U> CSVDataFeed<T, U> where
    T: Clone,
    U: serde::de::DeserializeOwned {


    fn is_orientated_correctly(&self) -> bool {

        if !self.data.is_empty() {
            if self.data.first().unwrap().time < self.data.last().unwrap().time {
                return false
            }
        }
        true
    }

    fn reverse_data(&mut self) {
        self.data.reverse()
    }

}


impl<T, U> DataFeed for CSVDataFeed<T, U>  where 
    U: serde::de::DeserializeOwned + IntoDataPoint<NumberType = T>,
    T: Clone {

    type NumberType = T;

    fn get_symbols(&self) -> &Vec<(SecuritySymbol, String)> {
        &self.symbols
    }

    fn connect(&mut self, sender: Sender<DataPoint<Self::NumberType>>, mode: RunMode) -> Result<(), DataError> {
        
        let mut reader = Reader::from_path(self.path.to_owned())?;

        if let Some((symbol, market)) = self.symbols.get(0) {
            for bar in reader.deserialize() {
                let bar: U = bar?;

                self.data.push(bar.to_datapoint(symbol.clone(), self.resolution))

            };
        };

        if !self.is_orientated_correctly() {
            self.reverse_data();
        }

        Ok(())
    }

    fn send_backtest(&mut self) -> Result<(), DataError> {

        let time = self.time.load(Ordering::Relaxed);

        loop {
            // TODO: Need to pop
            // Slice shouldnt output anyway after time has passed
            match self.data.last() {
                Some(val) => {
                    if val.time <= time {
                        self.sender.send(self.data.pop().unwrap())
                            .map_err(|err| DataError::FeedError(format!("Sender failed: {}", err)));
                    } else {
                        break
                    }
                },
                None => break,
            }
        }
        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.data.is_empty()
    }

}


pub struct CSVDataFeedBuilder<T, U> where T: Clone {

    path: PathBuf,
    symbols: Vec<(SecuritySymbol, String)>,
    data: Vec<DataPoint<T>>,
    time: Option<Arc<AtomicI64>>,
    resolution: Resolution,
    sender: Option<Sender<DataPoint<T>>>,
    fill_sender: Option<Sender<DataPoint<T>>>,
    mode: Option<RunMode>,

    deserialization_type: PhantomData<U>
}


impl<T, U> CSVDataFeedBuilder<T, U> where T: Clone {

    pub fn new(symbol: SecuritySymbol, market: &str, resolution: Resolution) -> CSVDataFeedBuilder<T, U> {

        let mut symbol_vec = Vec::new();
        symbol_vec.push((symbol, market.to_owned()));

        CSVDataFeedBuilder {
            path: PathBuf::new(),
            symbols: symbol_vec,
            data: Vec::new(),
            time: None,
            resolution,
            sender: None,
            fill_sender: None,
            mode: None,
            deserialization_type: PhantomData
        }
    }

    pub fn with_path(self, path: &str) -> Self {

        let mut tmp = self.path;
        tmp.push(path);

        Self {
            path: tmp,
            ..self
        }
    }

    // pub fn with_name(self, name: SecuritySymbol) -> Self {
    //     Self {
    //         symbols: name,
    //         ..self
    //     }
    // }

    pub fn with_resolution(self, resolution: Resolution) -> Self {
        Self {
            resolution: resolution,
            ..self
        }
    }



}

impl<T, U> DataFeedBuilder for CSVDataFeedBuilder<T, U> where T: Clone {

    type Output = CSVDataFeed<T, U>;

    type NumberType = T;

    fn build(self) -> Result<Self::Output, DataError> {
        // TODO: Add with_fill_sender
        Ok(CSVDataFeed {
            path: self.path,
            symbols: self.symbols,
            data: self.data,
            time: self.time
                .ok_or(DataError::IncompleteDataFeedBuilder(format!("Data Feed requires a shared time with the Engine")))?,
            resolution: self.resolution,
            sender: self.sender
                .ok_or(DataError::IncompleteDataFeedBuilder(format!("Data manager channel buffer not set")))?,
            mode: self.mode
                .ok_or(DataError::IncompleteDataFeedBuilder(format!("Data feed must have a run mode set")))?,
            phantom: self.deserialization_type
        })
    }

    fn with_sender(self, sender: Sender<DataPoint<T>>) -> Self {
        Self {
            sender: Some(sender),
            ..self
        }
    }

    fn with_fill_sender(self, sender: Sender<DataPoint<Self::NumberType>>) -> Self {
        Self {
            fill_sender: Some(sender),
            ..self
        }
    }

    fn with_mode(self, mode: RunMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    fn with_time(self, time: Arc<AtomicI64>) -> Self {
        Self {
            time: Some(time),
            ..self
        }
    }

    fn get_symbols(&self) -> &Vec<(SecuritySymbol, String)> {
        &self.symbols
    }

}

#[cfg(test)]
mod tests {
    use crate::data::data_providers::YahooFinanceTradeBar;

    use super::*;
    use std::sync::atomic::AtomicI64;
    use std::marker::PhantomData;
    use std::sync::mpsc::channel;

    fn setup_datapoint_vec() -> Vec<DataPoint<f64>> {

        let mut data = Vec::new();

        data.push(
            DataPoint { 
                symbol: crate::security::SecuritySymbol::Equity(String::from("TS")),
                time: 28000, 
                data: DataType::Bar(TradeBar::new(
                    1000.0,
                    4.53,
                    5.67,
                    2.45,
                    3.89,
                    27000,
                    28000,
                    false,
                    SecuritySymbol::Equity(String::from("TS")),
                    Resolution::Day,
                )),
                period: Resolution::Day
            }
        );

        data 

    }

    #[test]
    fn test_csvdatafeed_send_backtest() {

        // Arrange

        let (sender, receiver) = channel();

        let mut feed = CSVDataFeed {
            path: PathBuf::new(),
            symbols: vec![(SecuritySymbol::Equity(String::from("TS")), String::from("usa"))],
            data: setup_datapoint_vec(),
            time: Arc::new(AtomicI64::new(28000)),
            resolution: Resolution::Day,
            sender: sender,
            mode: RunMode::BackTest,
            phantom: PhantomData::<YahooFinanceTradeBar<f64>>,
        };

        let expected = setup_datapoint_vec().last().unwrap().clone();

        // Act
        let _ = feed.send_backtest();
        let result = receiver.recv_timeout(Duration::new(1,0)).unwrap();

        // Assert
        assert_eq!(result, expected)

    }

    #[test]
    fn test_csvdatafeed_send_backtest_no_bar() {

        // Arrange

        let (sender, receiver) = channel();

        let mut feed = CSVDataFeed {
            path: PathBuf::new(),
            symbols: vec![(SecuritySymbol::Equity(String::from("TS")), String::from("usa"))],
            data: setup_datapoint_vec(),
            time: Arc::new(AtomicI64::new(27000)),
            resolution: Resolution::Day,
            sender: sender,
            mode: RunMode::BackTest,
            phantom: PhantomData::<YahooFinanceTradeBar<f64>>,
        };

        let expected = std::sync::mpsc::RecvTimeoutError::Timeout;

        // Act
        let _ = feed.send_backtest();
        let result = receiver.recv_timeout(Duration::new(0,1000)).err().unwrap();

        // Assert
        assert_eq!(result, expected)

    }
}




// pub struct FeedError(String);

// impl fmt::Display for FeedError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Feed Error")
//     }
// }

// #[derive(Debug)]
// pub enum ConnectionError {
//     CSVError(Error)
// }

// impl From<Error> for ConnectionError {
//     fn from(err: Error) -> ConnectionError {
//         ConnectionError::CSVError(err)
//     }
// }

// impl fmt::Display for ConnectionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Failed to connect to Data Feed")
//     }
// }