use std::collections::HashMap;
use crate::DataNumberType;
use crate::error::Error;
use crate::security::Currency;

pub struct FXManager<T> where T: DataNumberType {

    data: HashMap<(Currency, Currency), T>

}

impl<T> FXManager<T> where T: DataNumberType {

    /// Provides an FXManager that holds all current fx rates
    pub fn new() -> Self {
        Self {
           data: HashMap::new()
        }
    }

    pub fn update(&mut self, source: Currency, target: Currency, rate: T) {
        self.data.insert((source, target), rate);
    }


}

#[derive(Debug, Eq, PartialEq)]
pub struct ExchangeRate<T> where T: DataNumberType {
    source: Currency,
    target: Currency,
    rate: T
}

impl<T> ExchangeRate<T> where T: DataNumberType {

    pub fn new(source: Currency, target: Currency, rate: T) -> Self {
        Self {
            source,
            target,
            rate
        }
    }

    pub fn exchange(&self, cash: T) -> T {
       self.rate * cash
    }

    pub fn chain(r1: &ExchangeRate<T>, r2: &ExchangeRate<T>) -> Result<ExchangeRate<T>, Error> {
        if r1.source == r2.source {
            Ok(
                Self {
                    source: r1.target,
                    target: r2.target,
                    rate: r2.rate/r1.rate
                }
            )
        } else if r1.source == r2.target {
            Ok(
                Self {
                    source: r1.target,
                    target: r2.source,
                    rate: <i8 as Into<T>>::into(1) / (r1.rate * r2.rate)
                }
            )
        } else if r1.target == r2.source {
            Ok(
                Self {
                    source: r1.source,
                    target: r2.target,
                    rate: r1.rate * r2.rate
                }
            )
        } else if r1.target == r2.target {
            Ok(
                Self {
                    source: r1.source,
                    target: r2.source,
                    rate: r1.rate / r2.rate
                }
            )
        } else {
            Err(Error::FXConversionError(String::from("exchange rate not chainable")))
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_rate_chain_same_sources() {

        // Arrange

        let rate_1 = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
           8.0
        );

        let rate_2 = ExchangeRate::new(
            Currency::USD,
            Currency::GBP,
           4.0
        );

        let expected = ExchangeRate::new(
            Currency::JPY,
            Currency::GBP,
           0.5
        );

        // Act
        let result = ExchangeRate::chain(&rate_1, &rate_2).unwrap();

        // Assert
        assert_eq!(result, expected)

    }


    #[test]
    fn test_exchange_rate_chain_source_to_target() {

        // Arrange

        let rate_1 = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            4.0
        );

        let rate_2 = ExchangeRate::new(
            Currency::GBP,
            Currency::USD,
            8.0
        );

        let expected = ExchangeRate::new(
            Currency::JPY,
            Currency::GBP,
            0.03125
        );

        // Act
        let result = ExchangeRate::chain(&rate_1, &rate_2).unwrap();

        // Assert
        assert_eq!(result, expected)

    }


    #[test]
    fn test_exchange_rate_chain_target_to_source() {

        // Arrange

        let rate_1 = ExchangeRate::new(
            Currency::USD,
            Currency::JPY,
            4.0
        );

        let rate_2 = ExchangeRate::new(
            Currency::JPY,
            Currency::GBP,
            8.0
        );

        let expected = ExchangeRate::new(
            Currency::USD,
            Currency::GBP,
            32.0
        );

        // Act
        let result = ExchangeRate::chain(&rate_1, &rate_2).unwrap();

        // Assert
        assert_eq!(result, expected)

    }


    #[test]
    fn test_exchange_rate_chain_same_target() {

        // Arrange

        let rate_1 = ExchangeRate::new(
        Currency::USD,
        Currency::JPY,
        4.0
        );

        let rate_2 = ExchangeRate::new(
        Currency::GBP,
        Currency::JPY,
        8.0
        );

        let expected = ExchangeRate::new(
        Currency::USD,
        Currency::GBP,
        0.5
        );

        // Act
        let result = ExchangeRate::chain(&rate_1, &rate_2).unwrap();

        // Assert
        assert_eq!(result, expected)

    }
}