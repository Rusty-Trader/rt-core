
use super::orders::Order;

use crate::DataNumberType;
use crate::data::DataPoint;
use crate::data::DataType;

pub mod simple_model;


pub trait SlippageModel {

    type NumberType: DataNumberType;
    
    fn get_slippage_approximation(&self, datapoint: &DataPoint<Self::NumberType>, order: impl Order) -> Self::NumberType;

    fn get_best_ask_price<T>(datapoint: &DataPoint<T>) -> T where T: DataNumberType {
        
        match datapoint.get_data() {
            DataType::Bar(data) => {
                return data.open
            },
            DataType::Tick(data) => {
                return data
            }
        }
    }
    
    fn get_best_bid_price<T>(datapoint: &DataPoint<T>) -> T where T: DataNumberType {
    
        match datapoint.get_data() {
            DataType::Bar(data) => {
                return data.open
            },
            DataType::Tick(data) => {
                return data
            }
        }
    }
}

