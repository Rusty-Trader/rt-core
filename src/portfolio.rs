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
                                    self.cash -= y.get_cost().into() - y.get_commission().into();
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