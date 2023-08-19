use std::fmt;


#[derive(Debug)]
pub enum DataError {
    ConnectionError,

    CSVError(csv::Error),

    IncompleteDataFeedBuilder(String),

    FeedError(String),

}

impl std::error::Error for DataError {}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::ConnectionError => 
                    write!(f, "Failed to connect Feed"),
            DataError::CSVError(err) =>
                write!(f, "{}", err),
            DataError::IncompleteDataFeedBuilder(d) =>
                write!(f, "{}", d),
            DataError::FeedError(d) => 
                write!(f, "{}", d),
        }
    }
}

impl From<csv::Error> for DataError {
    fn from(err: csv::Error) -> DataError {
        DataError::CSVError(err)
    }
}