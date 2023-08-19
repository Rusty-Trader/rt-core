use std::fmt::{self, write, Debug};
use std::collections::HashMap;
use std::ops::Index;
use chrono::NaiveDateTime;

use super::{Resolution, FillFwd};
use crate::utils::Merge;

#[derive(Debug, Clone)]
pub struct TradeBars<T> {

    data: HashMap<String, TradeBar<T>>,
}


impl<T> TradeBars<T> {

    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn from_bar(tradebar: TradeBar<T>) -> TradeBars<T> {
        let mut tmp = TradeBars::new();
        tmp.add(tradebar.symbol.clone().as_str(), tradebar);
        tmp
    }

    pub fn add(&mut self, symbol: &str, tradebar: TradeBar<T>) {
        self.data.insert(symbol.to_string(), tradebar);
    }

    pub fn symbols(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn contains_symbol(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn get_bar(&self, symbol: &str) -> Option<&TradeBar<T>> {
        self.data.get(symbol)
    }

    // pub fn map<U, F>(&self, f: F) -> Option<U> where F: FnOnce(T) -> U {

    // }

}

impl<T: fmt::Debug> fmt::Display for TradeBars<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T> Index<&'_ str> for TradeBars<T> {
    type Output = TradeBar<T>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.data[index]
    }

}

impl<T> Merge for TradeBars<T> {
    
    fn merge(&mut self, other: Self) {
        self.data.extend(other.data)
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeBar<T> {

    pub volume: T,

    pub open: T,

    pub high: T,

    pub low: T,

    pub close: T,

    pub start_time: i64,

    pub end_time: i64,

    pub is_fill_fwd: bool,

    pub symbol: String,

    pub period: Resolution,

}

impl<T> TradeBar<T> {
    pub fn new(
        volume: T,
        open: T,
        high: T,
        low: T,
        close: T,
        start_time: i64,
        end_time: i64,
        is_fill_fwd: bool,
        symbol: &str,
        period: Resolution
    ) -> TradeBar<T> {
        TradeBar {
            volume,
            open,
            high,
            low,
            close,
            start_time,
            end_time,
            is_fill_fwd,
            symbol: symbol.to_string(),
            period   
        }
    }
}

impl<T: fmt::Debug> fmt::Display for TradeBar<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "volume: {:?}, open: {:?}, high: {:?}, low: {:?}, close: {:?}, start time: {:?}, end_time: {:?}, fill fwd: {}, symbol: {}, period: {:?}", self.volume, self.open, self.high, self.low, self.close, NaiveDateTime::from_timestamp_millis(self.start_time), NaiveDateTime::from_timestamp_millis(self.end_time), self.is_fill_fwd, self.symbol, self.period)
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
            "Test Symbol",
            Resolution::Day
        );

        // Act
        
        // Assert
    }


} 