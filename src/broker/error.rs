use std::fmt;
use std::sync::mpsc::SendError;

use super::fill::engine::FillEngine;




#[derive(Debug)]
pub enum BrokerError {

    NextCycleError(String),

    FillEngineError(String),

    InsufficientFunds(String),

}


impl std::error::Error for BrokerError {}

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrokerError::NextCycleError(d) =>
                write!(f, "{}", d),
            BrokerError::FillEngineError(d) =>
                write!(f, "{}", d),
            BrokerError::InsufficientFunds(d) =>
                write!(f, "{}", d),
        }
    }
}