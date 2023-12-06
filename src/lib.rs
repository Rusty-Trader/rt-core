use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign, Rem};
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use rust_decimal::Decimal;

pub mod data;
pub mod error;
pub mod rtengine;
mod utils;
pub mod algorithm;
mod time;
pub mod portfolio;
pub mod broker;
pub mod security;
mod results;
mod lob;


pub trait NumberType: Copy + 
    Debug + 
    Add<Self, Output = Self> + 
    Sub<Self, Output = Self> + 
    Mul<Self, Output = Self> + 
    Div<Self, Output = Self> +
    Rem<Self, Output = Self> +
    AddAssign<Self> +
    SubAssign<Self> +
    PartialEq<Self> +
    PartialOrd<Self> +
    From<i8> {}

impl NumberType for f64 {}
impl NumberType for f32 {}

impl NumberType for Decimal {}

pub trait DataNumberType: NumberType {}

impl DataNumberType for f64 {}
impl DataNumberType for f32 {}

impl DataNumberType for Decimal {}

pub trait PortfolioNumberType: NumberType {}

impl PortfolioNumberType for f64 {}
impl PortfolioNumberType for f32 {}

impl PortfolioNumberType for Decimal {}


#[cfg(test)]
pub mod test_utils {
    use crate::data::{DataPoint, DataType, Resolution, tradebars::TradeBar};
    use crate::security::SecuritySymbol;


    pub fn setup_data_line_daily() -> Vec<DataPoint<f64>> {
        let mut output = Vec::new();

        output.push(
            DataPoint::new(
                crate::security::SecuritySymbol::Equity(String::from("AAPL")),
                1649116800000,
                DataType::Bar(TradeBar::new(
                    76468400.0,
                    174.570007,
                    178.490005,
                    174.440002,
                    178.440002,
                    1649030400000,
                    1649116800000,
                    false,
                    SecuritySymbol::Equity(String::from("AAPL")),
                    Resolution::Day
                )),
                Resolution::Day
            )
        );

        output.push(
            DataPoint::new(
                crate::security::SecuritySymbol::Equity(String::from("AAPL")),
                1649203200000,
                DataType::Bar(TradeBar::new(
                    73401800.0,
                    177.5,
                    178.300003,
                    174.419998,
                    175.059998,
                    1649116800000,
                    1649203200000,
                    false,
                    SecuritySymbol::Equity(String::from("AAPL")),
                    Resolution::Day
                )),
                Resolution::Day
            )
        );

        output.push(
            DataPoint::new(
                crate::security::SecuritySymbol::Equity(String::from("AAPL")),
                1649289600000,
                DataType::Bar(TradeBar::new(
                    89058800.0,
                    172.360001,
                    173.630005,
                    170.130005,
                    171.830002,
                    1649203200000,
                    1649289600000,
                    false,
                    SecuritySymbol::Equity(String::from("AAPL")),
                    Resolution::Day
                )),
                Resolution::Day
            )
        );

        output.push(
            DataPoint::new(
                crate::security::SecuritySymbol::Equity(String::from("AAPL")),
                1649376000000,
                DataType::Bar(TradeBar::new(
                    77594700.0,
                    171.160004,
                    173.360001,
                    169.850006,
                    172.139999,
                    1649289600000,
                    1649376000000,
                    false,
                    SecuritySymbol::Equity(String::from("AAPL")),
                    Resolution::Day
                )),
                Resolution::Day
            )
        );


        output
    }
}