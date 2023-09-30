use std::fmt;
use chrono::NaiveDateTime;

use super::{DataPoint, DataType, tradebars::{TradeBar, TradeBars}};

use crate::{DataNumberType, utils::Merge};
use crate::security::SecuritySymbol;

#[derive(Debug, Clone)]
pub struct Slice<T> {

    end_time: i64,

    // has_data: bool,

    bars: Option<TradeBars<T>>,

}

impl<T> Slice<T> where T: DataNumberType {

    pub fn new(time: i64) -> Self {
        Self {
            end_time: time,
            // has_data: false,
            bars: None
        }
    }

    pub fn has_data(&self) -> bool {
        // TODO: Make dynamic
        match &self.bars {
            Some(bars) => {bars.has_data()},
            None => false,
        }
    }

    pub fn add_bar(&mut self, symbol: SecuritySymbol, bar:TradeBar<T>) {
        match &mut self.bars {
            Some(bars) => bars.add(symbol, bar),
            None => {
                let mut tradebars = TradeBars::new();
                tradebars.add(symbol, bar);
                self.bars = Some(tradebars)
            },
        }
    }

    pub fn add_datapoint(&mut self, point: DataPoint<T>) {
        match point.get_data() {
            DataType::Bar(x) => {
                self.add_bar(point.get_symbol(), x)
            },
            _ => {}
        }
    }

    // pub fn from_bar(bar: TradeBar<T>) -> Slice<T> {
    //     Self {
    //         end_time: bar.end_time,
    //         has_data: true,
    //         bars: Some(TradeBars::from_bar(bar))
    //     }
    // }

    pub fn get_bar_by_symbol(&self, key: &SecuritySymbol) -> Option<&TradeBar<T>> {

        if let Some(v) = &self.bars {
            return v.get_bar(key)
        }
        None
    }

}

impl<T: fmt::Debug> fmt::Display for Slice<T> where T: DataNumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "end time: {:?}, has data: {}, bars: {:?}", NaiveDateTime::from_timestamp_millis(self.end_time), self.has_data(), self.bars)
    }
}

// impl<T> From<DataPoint<T>> for Slice<T> where T: Clone {
//     fn from(item: DataPoint<T>) -> Self {
//         match item.data {
//             // DataType::None => Slice::new(item.time),
//             DataType::Bar(bar) => {

//                 Slice::from_bar(bar)
//             },
//             DataType::Tick(_) => Slice::new(item.time)
//         }
//     }
// }

impl<T> Eq for Slice<T> {}

impl<T> PartialEq for Slice<T> {

    fn eq(&self, other: &Self) -> bool {
        self.end_time == other.end_time
    }

}

impl<T> PartialOrd for Slice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.end_time.cmp(&other.end_time))
    }
}


impl<T> Ord for Slice<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_time.cmp(&other.end_time)
    }
}

impl<T> Merge for Slice<T> {

    fn merge(&mut self, other: Self) {
        self.bars.merge(other.bars)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tradebars::TradeBar;
    use crate::test_utils::setup_data_line_daily;


    fn setup_tradebar() -> TradeBar<f64> {

        let input_data = setup_data_line_daily();

        match input_data.get(0).unwrap().clone().get_data() {
            DataType::Bar(x) => x,
            _ => panic!("Data type for test should be trade bars")
        }
    }

    #[test]
    fn test_has_data_false() {

        // Arrange
        let slice: Slice<f64> = Slice::new(1649116800000);

        let expected = false;

        // Act
        let result = slice.has_data();

        //Assert
        assert_eq!(result, expected)

    }

    #[test]
    fn test_has_data_true() {

        // Arrange
        let mut slice: Slice<f64> = Slice::new(1649116800000);

        let tradebar = setup_tradebar();

        slice.add_bar(SecuritySymbol::Equity(String::from("AAPL")), tradebar);

        let expected = true;

        // Act
        let result = slice.has_data();

        //Assert
        assert_eq!(result, expected)

    }

    #[test]
    fn test_add_bar() {

        // Arrange
        let mut slice: Slice<f64> = Slice::new(1649116800000);

        let tradebar = setup_tradebar();

        let expected = tradebar.clone();

        // Act
        slice.add_bar(SecuritySymbol::Equity(String::from("AAPL")), tradebar);

        let result = slice.bars.unwrap().get_bar(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap().clone();

        //Assert
        assert_eq!(result, expected)
    }

    #[test]
    fn test_add_datapoint() {

        // Arrange
        let mut input_data = setup_data_line_daily();

        let mut slice: Slice<f64> = Slice::new(1649116800000);

        let point = input_data.get(0).unwrap().clone();

        let tradebar = setup_tradebar();

        let expected = tradebar;

        // Act
        slice.add_datapoint(point);

        let result = slice.bars.unwrap().get_bar(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap().clone();

        //Assert
        assert_eq!(result, expected)

    }

    #[test]
    fn test_get_bar_by_symbol() {

        // Arrange
        let mut slice: Slice<f64> = Slice::new(1649116800000);

        let tradebar = setup_tradebar();

        let expected = tradebar.clone();

        // Act
        slice.add_bar(SecuritySymbol::Equity(String::from("AAPL")), tradebar);

        let result = slice.get_bar_by_symbol(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap().clone();

        //Assert
        assert_eq!(result, expected)

    }
}