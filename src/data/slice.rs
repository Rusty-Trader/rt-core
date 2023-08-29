use std::fmt;
use chrono::NaiveDateTime;

use super::{tradebars::{TradeBars, TradeBar}, DataPoint, DataType};
use super::FillFwd;

use crate::{utils::Merge, SecuritySymbol, DataNumberType};

#[derive(Debug, Clone)]
pub struct Slice<T> {

    end_time: i64,

    has_data: bool,

    bars: Option<TradeBars<T>>,

}

impl<T> Slice<T> where T: DataNumberType {

    pub fn new(time: i64) -> Self {
        Self {
            end_time: time,
            has_data: false,
            bars: None
        }
    }

    pub fn has_data(&self) -> bool {
        // TODO: Make dynamic
        self.has_data
    }

    pub fn add_bar(&mut self, bar:TradeBar<T>) {
        match &mut self.bars {
            Some(bars) => bars.add(bar.symbol.clone(), bar),
            None => {
                let mut tradebars = TradeBars::new();
                tradebars.add(bar.symbol.clone(), bar);
                self.bars = Some(tradebars)
            },
        }
    }

    pub fn add_datapoint(&mut self, point: DataPoint<T>) {
        match point.get_data() {
            DataType::Bar(x) => {
                self.add_bar(x)
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

impl<T: fmt::Debug> fmt::Display for Slice<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "end time: {:?}, has data: {}, bars: {:?}", NaiveDateTime::from_timestamp_millis(self.end_time), self.has_data, self.bars)
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
        self.has_data = self.has_data || other.has_data;
        self.bars.merge(other.bars)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tradebars::{TradeBar, TradeBars};
    use crate::test_utils::setup_data_line_daily;

    #[test]
    fn test_add_bar() {

        // Arrange

        let mut input_data = setup_data_line_daily();

        let mut slice = Slice::new(1649116800000);

        
        let tradebar = match input_data.get(0).unwrap().clone().get_data() {
            DataType::Bar(x) => x,
            _ => panic!("Data type for test should be trade bars")
        };

        let expected = tradebar.clone();

        // Act
        slice.add_bar(tradebar);

        let result = slice.bars.unwrap().get_bar(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap().clone();

        //Assert
        assert_eq!(result, expected)
    }

    #[test]
    fn test_add_datapoint() {

        // Arrange

        let mut input_data = setup_data_line_daily();

        let mut slice = Slice::new(1649116800000);

        let point = input_data.get(0).unwrap().clone();

        let tradebar = match point.get_data(){
            DataType::Bar(x) => x,
            _ => panic!("Data type for test should be trade bars")
        };

        let expected = tradebar;

        // Act
        slice.add_datapoint(point);

        let result = slice.bars.unwrap().get_bar(&SecuritySymbol::Equity(String::from("AAPL"))).unwrap().clone();

        //Assert
        assert_eq!(result, expected)

    }
}