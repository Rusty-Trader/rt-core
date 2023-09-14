pub mod data_providers;
pub mod datafeed;
pub mod datamanger;
pub mod tradebars;
pub mod error;
pub mod slice;

use tradebars::TradeBar;
use serde::{Deserialize, Serialize};
use csv::Reader;
use std::include_bytes;

use crate::{security::{Currency, SecurityType}};
use crate::security::{Equity, Security, SecuritySymbol};

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
    pub market: String,
    pub symbol: String,
    #[serde(rename(deserialize = "type"))]
    pub security_type: SecurityType,
    pub description: String,
    pub quote_currency: Currency,
    pub contract_multiplier: f64,
    pub minimum_price_variation: f64,
    pub lot_size: f64,
    pub market_ticker: Option<String>,
    pub minimum_order_size: Option<f64>,
    pub price_magnifier: Option<f64>
}

impl DataSymbolProperties {

    pub fn to_security(self) -> Security {
        match self.security_type {
            SecurityType::Equity => {
                Security::Equity(
                    Equity::new(
                        self.quote_currency,
                        self.lot_size
                    )
                )
            }
        }
    }
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
