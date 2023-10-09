use crate::rtengine::{RTEngine, BackTester};
use crate::data::slice::Slice;
use crate::broker::Broker;
use crate::DataNumberType;

pub trait Algo: Clone {

type NumberType: DataNumberType;

    fn on_data<T, U>(&self, slice: Slice<Self::NumberType>, engine: &mut RTEngine<T, U>) where
        T: Algo<NumberType = Self::NumberType>,
        U: Broker<NumberType = Self::NumberType, PortfolioNumberType = Self::NumberType> + BackTester;
}