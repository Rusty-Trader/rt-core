use std::fmt::{self, write, Debug};
use std::collections::HashMap;
use std::ops::Index;
use chrono::NaiveDateTime;
use crate::DataNumberType;

use super::{Resolution, FillFwd};
use crate::security::SecuritySymbol;
use crate::utils::Merge;

/// Holds a collection of TradeBar for different securities for a Slice in time.
#[derive(Debug, Clone)]
pub struct TradeBars<T> where T: DataNumberType {

    data: HashMap<SecuritySymbol, TradeBar<T>>,
}


impl<T> TradeBars<T> where T: DataNumberType {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    // pub fn from_bar(tradebar: TradeBar<T>) -> TradeBars<T> {
    //     let mut tmp = TradeBars::new();
    //     tmp.add(tradebar.symbol.clone().as_str(), tradebar);
    //     tmp
    // }

    pub fn add(&mut self, symbol: SecuritySymbol, tradebar: TradeBar<T>) {
        self.data.insert(symbol.clone(), tradebar);
    }

    pub fn symbols(&self) -> Vec<SecuritySymbol> {
        self.data.keys().cloned().collect()
    }

    pub fn contains_symbol(&self, key: &SecuritySymbol) -> bool {
        self.data.contains_key(key)
    }

    pub fn get_bar(&self, symbol: &SecuritySymbol) -> Option<&TradeBar<T>> {
        self.data.get(symbol)
    }

    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }

}

impl<T: fmt::Debug> fmt::Display for TradeBars<T> where T: DataNumberType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T> Index<&'_ SecuritySymbol> for TradeBars<T> where T: DataNumberType {
    type Output = TradeBar<T>;

    fn index(&self, index: &SecuritySymbol) -> &Self::Output {
        &self.data[index]
    }

}

impl<T> Merge for TradeBars<T> where T: DataNumberType {
    
    fn merge(&mut self, other: Self) {
        self.data.extend(other.data)
    }
}


/// Holds OHLCV data for a single time point.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TradeBar<T> where T: DataNumberType {

    /// Volume of the security traded within the time period
    pub volume: T,

    /// Open price of the security.
    pub open: T,

    /// Highest price reached by the security during the time period.
    pub high: T,

    /// Lowest price reached by the security during the time period.
    pub low: T,

    /// Closing price of the security.
    pub close: T,

    // TODO: Check that it is milliseconds
    /// Start of the period given as milliseconds from EPOCH.
    pub start_time: i64,

    /// End of the period given as milliseconds from EPOCH
    pub end_time: i64,

    /// Boolean indicating whether the values are actually from the previous bar but repeated due to no new data.
    pub is_fill_fwd: bool,

}

impl<T> TradeBar<T> where T: DataNumberType {
    pub fn new(
        volume: T,
        open: T,
        high: T,
        low: T,
        close: T,
        start_time: i64,
        end_time: i64,
        is_fill_fwd: bool,
    ) -> Self {
        TradeBar {
            volume,
            open,
            high,
            low,
            close,
            start_time,
            end_time,
            is_fill_fwd,
        }
    }

    pub fn get_spot(&self) -> T {
        self.close.clone()
    }
}

impl<T: fmt::Debug> fmt::Display for TradeBar<T> where T: DataNumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "volume: {:?}, open: {:?}, high: {:?}, low: {:?}, close: {:?}, start time: {:?}, end_time: {:?}, fill fwd: {}", self.volume, self.open, self.high, self.low, self.close, NaiveDateTime::from_timestamp_millis(self.start_time), NaiveDateTime::from_timestamp_millis(self.end_time), self.is_fill_fwd)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn add() {
        // Arrange
        let trade_bar = TradeBar::new(
            dec!(1000),
            dec!(1.3),
            dec!(1.6),
            dec!(1.1),
            dec!(1.2),
            1001,
            1002,
            false,
        );

        // Act
        
        // Assert
    }


} 