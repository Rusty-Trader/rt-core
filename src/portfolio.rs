use std::{collections::HashMap, hash::Hash};

use crate::{broker::orders::FilledOrder, PortfolioNumberType, DataNumberType, Security};
use crate::broker::orders::{Side, OrderError};

pub struct Portfolio<T, F> where F: DataNumberType {

    // TODO: Add support for foreign cash
    cash: T,

    holdings: HashMap<Security, T>,

    filled_orders: HashMap<String, Result<FilledOrder<F>, OrderError<F>>>


}


impl<T, F> Portfolio<T, F> where T: PortfolioNumberType, F: DataNumberType {

    pub fn new() -> Self {
        Self {
            cash: <i8 as Into<T>>::into(1),
            holdings: HashMap::new(),
            filled_orders: HashMap::new()
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
                                    *x += y.get_volume().into();
                                    self.cash -= y.get_cost().into() + y.get_commission().into();
                                },
                                Side::Sell => {
                                    *x -= y.get_volume().into();
                                    self.cash += y.get_cost().into() - y.get_commission().into();
                                }   
                            }
                        },
                        None => {
                            match y.get_side() {
                                Side::Buy => {
                                    self.holdings.insert(y.get_symbol(), y.get_volume().into());
                                    self.cash -= y.get_cost().into() - y.get_commission().into();
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

        let order: MarketOrder<f64> = MarketOrder::new("1", Security::Equity(String::from("Test")), 1000, 1000.0, Side::Buy);

        let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();
        orders.push(Ok(FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false)
        ));


        let expected = 3999.0;

        // Act
        portfolio.update_holdings(orders);
        let result = portfolio.cash;

        // Assert

        assert_eq!(result, expected);

    }


    #[test]
    fn test_update_holdings_sell_order() {

        // Arrange
        let mut portfolio: Portfolio<f64, f64> = Portfolio::new();

        portfolio.cash = 10000.0;

        let order: MarketOrder<f64> = MarketOrder::new("1", Security::Equity(String::from("Test")), 1000, 1000.0, Side::Sell);

        let mut orders: Vec<Result<FilledOrder<f64>, _>> = Vec::new();
        orders.push(Ok(FilledOrder::new(
            OrderType::MarketOrder(order),
            1000,
            1000.0,
            6.0,
            1.0,
            false)
        ));


        let expected_cash = 15999.0;
        let expected_holdings = 0.0;

        // Act
        portfolio.update_holdings(orders);
        let result = portfolio.cash;

        // Assert

        assert_eq!(result, expected);

    }
}