use rust_decimal::{Decimal};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Days, Months, Datelike};
use crate::DataNumberType;

use crate::security::SecuritySymbol;

use super::{TradeBar, Resolution, DataPoint, DataType};

pub trait IntoTradeBar {

    type NumberType: DataNumberType;

    fn to_tradebar(&self) -> TradeBar<Self::NumberType>;
}

pub trait IntoDataPoint {

    type NumberType: DataNumberType;

    fn to_datapoint(self, symbol: SecuritySymbol, period: Resolution) -> DataPoint<Self::NumberType>;
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct YahooFinanceTradeBar<T> {

    #[serde(rename(deserialize = "Date"))]
    date: NaiveDate,
    #[serde(rename(deserialize = "Open"))]
    open: T,
    #[serde(rename(deserialize = "High"))]
    high: T,
    #[serde(rename(deserialize = "Low"))]
    low: T,
    #[serde(rename(deserialize = "Close"))]
    close: T,
    #[serde(rename(deserialize = "Adj Close"))]
    adj_close: T,
    #[serde(rename(deserialize = "Volume"))]
    volume: T,
    #[serde(skip)]
    #[serde(default = "period_default")]
    period: Resolution

}

fn period_default() -> Resolution {
    Resolution::Day
}

impl<T> IntoDataPoint for YahooFinanceTradeBar<T> where T: DataNumberType {

    type NumberType = T;

    fn to_datapoint(self, symbol: SecuritySymbol, resolution: Resolution) -> DataPoint<Self::NumberType> {

        let mut tmp: TradeBar<Self::NumberType> = TradeBar::from(self);
        tmp.period = resolution;

        DataPoint {
            symbol,
            time: tmp.end_time,
            data: DataType::Bar(tmp),
            period: resolution
        }
    }
}

impl<T> From<YahooFinanceTradeBar<T>> for TradeBar<T> where T: DataNumberType {
    fn from(item: YahooFinanceTradeBar<T>) -> Self {
        TradeBar {
            volume: item.volume,
            open: item.open,
            high: item.high,
            low: item.low,
            close: item.close,
            start_time: item.date.and_hms_opt(0, 0, 0).unwrap().timestamp() as i64 * 1000,
            end_time: add_day_to_date(item.date).and_hms_opt(0, 0, 0).unwrap().timestamp() as i64 * 1000,
            is_fill_fwd: true,
            symbol: SecuritySymbol::Equity(String::from("")),
            period: item.period,
        }
    }
}

// impl<T: Copy> IntoTradeBar for YahooFinanceTradeBar<T> {

//     type NumberType = T;

//     fn to_tradebar(&self) -> TradeBar<Self::NumberType> {
//         TradeBar::from(*self)
//     }
// }

fn add_day_to_date(date: NaiveDate) -> NaiveDate {
    // let rollover_date = date.clone();
    date.checked_add_days(Days::new(1))
        .unwrap_or(
            date
                .checked_add_months(Months::new(1))
                .unwrap()
                .checked_sub_days(Days::new(date.day() as u64 - 1))
                .unwrap()
        )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_day_to_date_middle_of_month() {
        // Arrange
        let x = NaiveDate::from_ymd_opt(2022, 03, 15).unwrap();

        let expected = NaiveDate::from_ymd_opt(2022, 03, 16).unwrap();

        // Act
        let result = add_day_to_date(x);

        // Assert
        assert_eq!(result, expected)
    }

    #[test]
    fn test_add_day_to_date_end_of_month() {
        // Arrange
        let x = NaiveDate::from_ymd_opt(2022, 03, 31).unwrap();

        let expected = NaiveDate::from_ymd_opt(2022, 04, 01).unwrap();

        // Act
        let result = add_day_to_date(x);

        // Assert
        assert_eq!(result, expected)
    }

}