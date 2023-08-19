use std::ops::Mul;

use super::SlippageModel;
use super::super::orders::Order;

use crate::DataNumberType;
use crate::broker::orders::Side;
use crate::data::DataPoint;

pub struct SimpleSlippageModel<T> where T: DataNumberType {

    slippage: T

}

impl<T> SimpleSlippageModel<T> where T: DataNumberType {
    pub fn new(slippage: T) -> Self {
        Self {
            slippage
        }
    }
}

impl<T> SlippageModel for SimpleSlippageModel<T> where T: DataNumberType {

    type NumberType = T;

    fn get_slippage_approximation(&self, datapoint: &DataPoint<Self::NumberType>, order: impl Order) -> Self::NumberType {
        
        match order.get_side() {
            Side::Buy => {
                return Self::get_best_ask_price(datapoint) * (<i8 as Into<T>>::into(1) + self.slippage)
            },
            Side::Sell => {
                return Self::get_best_bid_price(datapoint) * (<i8 as Into<T>>::into(1) - self.slippage)
            }
        }

    }
}
