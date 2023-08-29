pub mod data_providers;
pub mod datafeed;
pub mod datamanger;
pub mod tradebars;
pub mod error;
pub mod slice;

use tradebars::TradeBar;
use serde::{Serialize, Deserialize};
use csv::Reader;
use std::include_bytes;

use crate::{SecuritySymbol, security::{Currency, SecurityType}};

use self::error::DataError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataPoint<T: Clone> {
    symbol: SecuritySymbol,
    time: i64,
    data: DataType<T>,
    period: Resolution,
}

impl<T> DataPoint<T> where T: Clone {

    pub fn new(symbol: SecuritySymbol, time: i64, data: DataType<T>, period: Resolution) -> Self {
        Self { 
            symbol,
            time,
            data,
            period
        }
    }
    
    pub fn get_symbol(&self) -> SecuritySymbol {
        self.symbol.clone()
    }

    pub fn get_time(&self) -> i64 {
        self.time
    }

    pub fn get_period(&self) -> Resolution {
        self.period
    }

    pub fn get_data(&self) -> DataType<T> {
        self.data.clone()
    }


}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType<T> {
    Bar(TradeBar<T>),
    Tick(T),
    // None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Tick,
    Second = 1000,
    Minute = 60000,
    Hour = 3600000,
    Day = 86400000,
    Week = 604800000
}


pub trait FillFwd {
    fn fill_fwd(self) -> Self;
}


#[derive(Debug, Deserialize, Clone)]
pub struct DataSymbolProperties {
    market: String,
    symbol: String,
    #[serde(rename(deserialize = "type"))]
    security_type: SecurityType,
    description: String,
    quote_currency: Currency,
    contract_multiplier: f64,
    minimum_price_variation: f64,
    lot_size: f64,
    market_ticker: Option<String>,
    minimum_order_size: Option<f64>,
    price_magnifier: Option<f64>
}

pub fn deserialize_symbol_properties() -> Result<Vec<DataSymbolProperties>, DataError> {

    let bytes = include_bytes!("data/data-symbol-properties.csv");

    let mut tmp: Vec<DataSymbolProperties> = Vec::new();

    let mut reader = Reader::from_reader(&bytes[..]);

    for property in reader.deserialize() {
        
        tmp.push(property?);
    }

    Ok(tmp)

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_symbol_properties() {

        // Arrange

        // Act
        let result = deserialize_symbol_properties();
        println!("{:?}", result);
        // Assert
        // assert_ne!()

    }

}
