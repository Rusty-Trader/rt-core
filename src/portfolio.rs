use std::collections::HashMap;
use std::hash::Hash;

use crate::{broker::orders::FilledOrder, PortfolioNumberType, DataNumberType, SecuritySymbol};
use crate::broker::orders::{Side, OrderError};


/// A Holding stores information about the security that a portfolio contains.
#[derive(Debug, PartialEq, PartialOrd, Eq)]
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
}

pub struct Portfolio<T, F> where
    T: PortfolioNumberType,
    F: DataNumberType {

    // TODO: Add support for foreign cash
    cash: T,

    holdings: HashMap<SecuritySymbol, Holding<T>>,

    filled_orders: HashMap<String, Result<FilledOrder<F>, OrderError<F>>>,

    // registered_securities: HashMap<SecuritySymbol, SecurityDetails>


}


pub enum SecurityDetails {

    

}


impl<T, F> Portfolio<T, F> where T: PortfolioNumberType, F: DataNumberType {

    pub fn new() -> Self {
        Self {
            cash: <i8 as Into<T>>::into(1),
            holdings: HashMap::new(),
            filled_orders: HashMap::new(),
        }
    }

    pub fn update_holdings(&mut self, orders: Vec<Result<FilledOrder<F>, OrderError<F>>>) where
        F: DataNumberType + Into<T> {

        for order in orders {
            match &order {
                Ok(y) => {
                    match self.holdings.get_mut(&y.get_symbol()) {
                        Some(x) => {
                            match y.get_side() {
                                Side::Buy => {
                                    x.add(y.get_volume().into());
                                    self.cash -= y.get_cost().into() + y.get_commission().into();
                                },
                                Side::Sell => {
                                    x.sub(y.get_volume().into());
                                    self.cash += y.get_cost().into() - y.get_commission().into();
                                }   
                            }
                        },
                        None => {
                            match y.get_side() {
                                Side::Buy => {
                                    self.holdings.insert(y.get_symbol(), Holding::new(y.get_symbol(), y.get_volume().into()));
                                    self.cash -= y.get_cost().into() + y.get_commission().into();
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
    }
    
    pub fn get_cash(&self) -> T {
        self.cash
    }

    pub fn set_cash(&mut self, cash: F) where F: Into<T> {
        self.cash = cash.into()
    }

    pub fn get_filled_orders(&self) -> &HashMap<String, Result<FilledOrder<F>, OrderError<F>>> {
        &self.filled_orders
    }

    pub fn get_holding(&self, symbol: SecuritySymbol) -> Option<&Holding<T>> {
        self.holdings.get(&symbol)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker::orders::{FilledOrder, MarketOrder, OrderType};

    #[test]
    fn test_update_holdings_buy_order() {

        // Arrange
        let mut portfolio: Portfolio<f64, f64> = Portfolio::new();

        portfolio.cash = 10000.0;

        let order: MarketOrder<f64> = MarketOrder::new("1", SecuritySymbol::Equity(String::from("Test")), 1000, 1000.0, Side::Buy);

        let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();
        orders.push(Ok(FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false)
        ));


        let expected_cash = 3999.0;

        let expected_holdings = &Holding::Equity(1000.0);


        // Act
        portfolio.update_holdings(orders);
        let result_cash = portfolio.cash;
        let result_holdings = portfolio.holdings.get(&SecuritySymbol::Equity(String::from("Test"))).unwrap();

        // Assert

        assert_eq!(result_cash, expected_cash);
        assert_eq!(result_holdings, expected_holdings)

    }


    #[test]
    fn test_update_holdings_sell_order() {

        // Arrange
        let mut portfolio: Portfolio<f64, f64> = Portfolio::new();

        portfolio.cash = 0.0;

        let mut portfolio_map = HashMap::new();
        portfolio_map.insert(SecuritySymbol::Equity(String::from("Test")), Holding::Equity(1000.0));

        portfolio.holdings = portfolio_map;

        let order: MarketOrder<f64> = MarketOrder::new("1", SecuritySymbol::Equity(String::from("Test")), 1000, 1000.0, Side::Sell);

        let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();

        let filled_order = FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false
        );

        orders.push(Ok(filled_order));

        let expected_cash = 5999.0;
        let expected_holdings = &Holding::Equity(0.0);

        // Act
        portfolio.update_holdings(orders);
        let result_cash = portfolio.cash;
        let result_holdings = portfolio.holdings.get(&SecuritySymbol::Equity(String::from("Test"))).unwrap();

        // Assert

        assert_eq!(result_cash, expected_cash);
        assert_eq!(result_holdings, expected_holdings);

    }
}