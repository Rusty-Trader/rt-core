use std::collections::HashMap;

use crate::security::{Currency, Security, SecuritySymbol};
use crate::{broker::orders::FilledOrder, DataNumberType, PortfolioNumberType};
use crate::broker::orders::{OrderError, Side};
// use crate::error::Error;


/// A Holding stores information about the security that a portfolio contains.
#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub enum Holding<T> where T: PortfolioNumberType {
    Equity(T)
}


impl<T> Holding<T> where T: PortfolioNumberType {

    fn add(&mut self, volume: T) {
        match self {
            Self::Equity(x) => {
                *x += volume
            },
        }
    }

    fn sub(&mut self, volume: T) {
        match self {
            Self::Equity(x) => {
                *x -= volume
            }
        }
    }

    fn new(symbol: SecuritySymbol, volume: T) -> Self {
        match symbol {
            SecuritySymbol::Equity(_) => {
                return Self::Equity(volume)
            }
        }
    }

    pub fn get_volume(&self) -> T {
        match self {
            Self::Equity(amnt) => *amnt
        }
    }

}

pub struct Cash<T> where T: PortfolioNumberType {
    volume: T,
    rate: T
}

impl<T> Cash<T> where T: PortfolioNumberType {

    pub fn new(volume: T, rate: T) -> Self {
        Self {
            volume,
            rate
        }
    }
    
}

/// Portfolio that stores information about the algorithm's current cash balance and security holdings.
pub struct Portfolio<T, F> where
    T: DataNumberType,
    F: PortfolioNumberType {

    // TODO: Add support for foreign cash - Need to connect this to data manager and include an fx table and triangulation
    /// Reporting currency of the portfolio.
    reporting_currency: Currency,

    /// HashMap to store the number of units of each currency
    cash_holdings: HashMap<Currency, F>,

    /// HashMap to store the number of units of each security
    holdings: HashMap<SecuritySymbol, Holding<F>>,

    /// HashMap that stores all submitted orders that have either been successfully filled or have raised an error.
    filled_orders: HashMap<String, Result<FilledOrder<T>, OrderError<T>>>,

    /// A HashMap that stores a database of security information such as the denominated currency.
    registered_securities: HashMap<SecuritySymbol, Security>


}


impl<T, F> Portfolio<T, F> where
    T: DataNumberType,
    F: PortfolioNumberType {

    /// Returns a portfolio with the reporting currency set
    pub fn new(reporting_currency: Currency) -> Self {

        let mut tmp_cash = HashMap::new();

        tmp_cash.insert(reporting_currency, <i8 as Into<F>>::into(1));

        Self {
            reporting_currency,
            cash_holdings: tmp_cash,
            holdings: HashMap::new(),
            filled_orders: HashMap::new(),
            registered_securities: HashMap::new()
        }
    }

    // TODO: move error handing of order into seperate function
    /// Updates the holdings of a portfolio when an order is filled
    pub fn update_holding(&mut self, order: Result<FilledOrder<T>, OrderError<T>>) where
        T: DataNumberType + Into<F> {

        // for order in orders {
        match order.clone() {
            Ok(y) => {
                let order_ccy: Security = self.security_details(&y.get_symbol()).unwrap().clone(); // TODO: Error handling
                match self.holdings.get_mut(&y.get_symbol()) {
                    Some(x) => {
                        match y.get_side() {
                            Side::Buy => {
                                x.add(y.get_volume().into());
                                self.sub_cash(&y, order_ccy.get_currency())
                            },
                            Side::Sell => {
                                x.sub(y.get_volume().into());
                                self.add_cash(&y, order_ccy.get_currency())
                            }
                        }
                    },
                    None => {
                        match y.get_side() {
                            Side::Buy => {
                                self.holdings.insert(y.get_symbol(), Holding::new(y.get_symbol(), y.get_volume().into()));
                                self.sub_cash(&y, order_ccy.get_currency());
                            },
                            Side::Sell => {}   
                        }  
                    }
                }

                self.filled_orders.insert(y.get_id(), order.clone());
            
            },
            Err(e) => {
                self.filled_orders.insert(e.get_id(), order.clone());
            }
        }
    }

    fn add_cash(&mut self, order: &FilledOrder<T>, currency : Currency) where T: DataNumberType + Into<F> {
        match self.cash_holdings.get_mut(&currency) {
            Some(cash) => {
                *cash += order.get_cost().into() - order.get_commission().into();
            },
            None => {}
        }
    }

    fn sub_cash(&mut self, order: &FilledOrder<T>, currency: Currency) where T: DataNumberType + Into<F> {
        match self.cash_holdings.get_mut(&currency) {
            Some(cash) => {
                *cash -= order.get_cost().into() + order.get_commission().into();
            },
            None => {}
        }
    }
    
    pub fn get_cash(&self, currency: Currency) -> Option<&F> {
        self.cash_holdings.get(&currency)
    }

    pub fn set_cash(&mut self, currency: Currency, cash: T) where T: Into<F> {
        self.cash_holdings.insert(currency, cash.into());
    }

    pub fn get_filled_orders(&self) -> &HashMap<String, Result<FilledOrder<T>, OrderError<T>>> {
        &self.filled_orders
    }

    pub fn get_holding(&self, symbol: SecuritySymbol) -> Option<Holding<F>> {
        self.holdings.get(&symbol).map(|x| x.to_owned())
    }

    pub fn register_security(&mut self, symbol: SecuritySymbol, details: Security) {
        self.registered_securities.insert(symbol, details);
    }

    pub fn security_details(&self, symbol: &SecuritySymbol) -> Option<&Security> {
        self.registered_securities.get(symbol)
    }

    pub fn is_registered(&self, symbol: SecuritySymbol) -> bool {
        self.registered_securities.contains_key(&symbol)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker::orders::{FilledOrder, MarketOrder, OrderType};
    use crate::security::Equity;

    #[test]
    fn test_update_holdings_buy_order() {

        // Arrange
        let mut portfolio: Portfolio<f64, f64> = Portfolio::new(Currency::USD);

        portfolio.cash_holdings.insert(Currency::USD, 10000.0);

        portfolio.register_security(
            SecuritySymbol::Equity(String::from("Test")),
            Security::Equity(
                Equity::new(
                    Currency::USD,
                    0.01
                )
            )
        );

        let order: MarketOrder<f64> = MarketOrder::new("1", SecuritySymbol::Equity(String::from("Test")), 1000, 1000.0, Side::Buy);

        // let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();
        let filled_order = Ok(FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false)
        );


        let expected_cash = 3999.0;

        let expected_holdings = &Holding::Equity(1000.0);


        // Act
        portfolio.update_holding(filled_order);
        let result_cash = *portfolio.get_cash(Currency::USD).unwrap();
        let result_holdings = portfolio.holdings.get(&SecuritySymbol::Equity(String::from("Test"))).unwrap();

        // Assert

        assert_eq!(result_cash, expected_cash);
        assert_eq!(result_holdings, expected_holdings)

    }


    #[test]
    fn test_update_holdings_sell_order() {

        // Arrange
        let mut portfolio: Portfolio<f64, f64> = Portfolio::new(Currency::USD);

        portfolio.cash_holdings.insert(Currency::USD, 0.0);

        portfolio.register_security(
            SecuritySymbol::Equity(String::from("Test")),
            Security::Equity(
                Equity::new(
                    Currency::USD,
                    0.01
                )
            )
        );

        let mut portfolio_map = HashMap::new();
        portfolio_map.insert(SecuritySymbol::Equity(String::from("Test")), Holding::Equity(1000.0));

        portfolio.holdings = portfolio_map;

        let order: MarketOrder<f64> = MarketOrder::new("1", SecuritySymbol::Equity(String::from("Test")), 1000, 1000.0, Side::Sell);

        // let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();

        let filled_order = Ok(FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false
        ));

        // orders.push(Ok(filled_order));

        let expected_cash = 5999.0;
        let expected_holdings = &Holding::Equity(0.0);

        // Act
        portfolio.update_holding(filled_order);
        let result_cash = *portfolio.get_cash(Currency::USD).unwrap();
        let result_holdings = portfolio.holdings.get(&SecuritySymbol::Equity(String::from("Test"))).unwrap();

        // Assert

        assert_eq!(result_cash, expected_cash);
        assert_eq!(result_holdings, expected_holdings);

    }
}