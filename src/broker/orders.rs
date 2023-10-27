use crate::DataNumberType;
use crate::security::SecuritySymbol;


pub trait Order {

    type NumberType;

    // fn is_filled(&self) -> bool;

    // fn set_timestamp(&mut self, timestamp: i64);
    /// Returns the order id.
    fn get_id(&self) -> String;

    /// Returns the symbol of the security that the order is for.
    fn get_symbol(&self) -> SecuritySymbol;

    /// Returns the timestamp of when the order was placed
    fn get_timestamp(&self) -> i64;

    /// Returns the number of units of the underlying security that the order is for.
    fn get_volume(&self) -> Self::NumberType;

    /// Returns an enum indicating whether the order is a "Buy" or "Sell"
    fn get_side(&self) -> Side;
}

/// An enum that represents the order type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType<T> where T: DataNumberType {
    MarketOrder(MarketOrder<T>)
}

impl<T> Order for OrderType<T> where T: DataNumberType {

    type NumberType = T;

    // fn is_filled(&self) -> bool {
    //     match self {
    //         OrderType::MarketOrder(x) => x.is_filled,
    //     }
    // }
    // fn set_timestamp(&mut self, timestamp: i64) {
    //     match self {
    //         OrderType::MarketOrder(x) => x.set_timestamp(timestamp)
    //     }
    // }

    fn get_id(&self) -> String {
        match self {
            OrderType::MarketOrder(x) => x.get_id()
        }
    }

    fn get_symbol(&self) -> SecuritySymbol {
        match self {
            OrderType::MarketOrder(x) => x.get_symbol()
        }
    }

    fn get_timestamp(&self) -> i64 {
        match self {
            OrderType::MarketOrder(x) => x.get_timestamp()
        }
    }

    fn get_volume(&self) -> Self::NumberType {
        match self {
            OrderType::MarketOrder(x) => x.get_volume()
        }
    }

    fn get_side(&self) -> Side {
        match self {
            OrderType::MarketOrder(x) => x.get_side()
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketOrder<T> where T: Clone {

    id: String,

    symbol: SecuritySymbol,

    timestamp: i64,

    volume: T,

    side: Side
}


impl<T> MarketOrder<T> where T: Clone {

    pub fn new(id: &str, symbol: SecuritySymbol, timestamp: i64, volume: T, side: Side) -> Self {
        Self { 
            id: String::from(id),
            symbol,
            timestamp,
            volume,
            side
        }
    }

}


impl<T> Order for MarketOrder<T> where T: Clone {

    type NumberType = T;

    // fn is_filled(&self) -> bool {
    //     self.is_filled
    // }

    // fn set_timestamp(&mut self, timestamp: i64) {
    //     self.timestamp = timestamp;
    // }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_symbol(&self) -> SecuritySymbol {
        self.symbol.clone()
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_volume(&self) -> Self::NumberType {
        self.volume.clone()
    }

    fn get_side(&self) -> Side {
        self.side
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilledOrder<T> where T: DataNumberType {

    order: OrderType<T>,

    timestamp: i64,

    volume: T,

    price: T,

    commission: T,

    partial: bool,
}


impl<T> FilledOrder<T> where T: DataNumberType {

    pub fn new(order: OrderType<T>, timestamp: i64, volume: T, price: T, commission: T, partial: bool) -> Self {
        Self {
            order,
            timestamp,
            volume,
            price,
            commission,
            partial
        }
    }

    pub fn get_id(&self) -> String {
        self.order.get_id()
    }

    pub fn get_symbol(&self) -> SecuritySymbol {
        self.order.get_symbol()
    }

    pub fn get_side(&self) -> Side {
        self.order.get_side()
    }

    pub fn get_volume(&self) -> T {
        self.volume
    }

    pub fn get_price(&self) -> T {
        self.price
    }

    pub fn get_cost(&self) -> T {
        self.price * self.volume
    }

    pub fn get_commission(&self) -> T {
        self.commission
    }

}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderError<T> where T: DataNumberType {

    order: OrderType<T>,

    timestamp: i64,

    error: String,

}

impl<T> OrderError<T> where T: DataNumberType {
    pub fn new(order: OrderType<T>, timestamp: i64, error: &str) -> Self {
        Self {
            order,
            timestamp,
            error: String::from(error)
        }
    }

    pub fn get_id(&self) -> String {
        self.order.get_id()
    }

    pub fn get_symbol(&self) -> SecuritySymbol {
        self.order.get_symbol()
    }
}

impl<T> std::fmt::Display for OrderError<T> where T: DataNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<T> std::error::Error for OrderError<T> where T: DataNumberType {}

/// Enum representing the side of the order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell
}


