use std::fmt;
use std::fmt::write;

use crate::broker::error::BrokerError;
use crate::data::error::DataError;



#[derive(Debug)]
pub enum Error {


    IncompleteBuilder(String),

    BrokerError(BrokerError),

    DataError(DataError),

    OrderError(String),

    FXConversionError(String)

}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IncompleteBuilder(d) =>
                write!(f, "{}", d),
            Error::BrokerError(err) =>
                write!(f, "{}", err),
            Error::DataError(err) =>
                write!(f, "{}", err),
            Error::OrderError(d) =>
                write!(f, "{}", d),
            Error::FXConversionError(d) =>
                write!(f, "{}", d)
        }
    }
}

impl From<BrokerError> for Error {
    fn from(err: BrokerError) -> Error {
        Error::BrokerError(err)
    }
}

impl From<DataError> for Error {
    fn from(err: DataError) -> Error {
        Error::DataError(err)
    }
}
