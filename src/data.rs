pub mod data_providers;
pub mod datafeed;
pub mod datamanger;
pub mod tradebars;
pub mod error;
pub mod slice;

use tradebars::TradeBar;

use crate::SecuritySymbol;

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