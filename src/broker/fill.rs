use std::collections::HashMap;
use std::hash::Hash;

use crate::{PortfolioNumberType, DataNumberType, Security};
use crate::utils::Merge;

pub mod engine;

#[derive(Debug, Clone)]
pub struct PortfolioData<T> where T: DataNumberType {
    
    pub cash: T,

    pub holdings: HashMap<Security, T>

}

impl<T> PortfolioData<T> where T: DataNumberType {

    pub fn new() -> Self {
        Self {
            cash: <i8 as Into<T>>::into(0),
            holdings: HashMap::new()
        }
    }

    pub fn get_cash(&self) -> T {
        self.cash
    }

    pub fn get_holding_amount(&self, holding: Security) -> Option<&T> {
        self.holdings.get(&holding)
    }
}

impl<T> Merge for PortfolioData<T> where T: DataNumberType {

    fn merge(&mut self, other: Self) {
        self.cash = other.cash
    }
}

