use std::collections::VecDeque;
use crate::broker::orders::Side;
use crate::DataNumberType;
use crate::error::Error;
use crate::security::SecuritySymbol;

#[derive(Debug, Copy, Clone)]
pub struct LOBOrder<T> where T: DataNumberType {
    side: Side,
    volume: T,
    price: T,
    trader: i64,
    order_id: i64
}


pub struct OrderBook<T> where T: DataNumberType {

    symbol: SecuritySymbol,
    buy_price_points: VecDeque<VecDeque<LOBOrder<T>>>,
    sell_price_points: VecDeque<VecDeque<LOBOrder<T>>>,
    bid_price: T,
    ask_price: T,
    lot_size: T
}

impl<T> OrderBook<T> where T: DataNumberType + From<i8> {

    pub fn new(symbol: SecuritySymbol, lot_size: T) -> Self {
        Self {
            symbol,
            buy_price_points: VecDeque::new(),
            sell_price_points: VecDeque::new(),
            bid_price: <i8 as Into<T>>::into(0),
            ask_price: <i8 as Into<T>>::into(0),
            lot_size
        }
    }


    fn price_to_index(&self, price: T, side: Side) -> usize where T: Into<usize> {
        match side {
            Side::Buy => {
                ((self.bid_price - price) / self.lot_size).into()
            },
            Side::Sell => {
                ((price - self.ask_price) / self.lot_size).into()
            }
        }
    }

    fn is_divisible_by_lot_size(&self, price: T) -> bool where T: Into<i8> {
        price % self.lot_size == <i8 as Into<T>>::into(0)
    }

    fn order_match_info(&mut self, order: &LOBOrder<T>) where T: Into<usize> + Into<i8> {

    }

    fn limit_order_buy(&mut self, order: &LOBOrder<T>) -> Result<(), Error> where T: Into<usize> + Into<i8> {
        let mut order_vol = order.volume;

        for i in 0..=self.price_to_index(order.price, Side::Buy) {
            // Access the index add the ask price and move up
            if let Some(matched_index) = self.sell_price_points.get_mut(i) {

                for i in 0..=matched_index.len() {

                    if let Some(matched_order) = matched_index.get_mut(i) {
                        if order_vol < matched_order.volume {
                            matched_order.volume -= order_vol;
                            self.order_match_info(order);
                        } else {
                            order_vol -= matched_order.volume;
                            self.order_match_info(order);
                        }
                    }
                }

            }
        }

        Ok(())

    }

    pub fn add_limit_order(&mut self, order: LOBOrder<T>) -> Result<(), Error> where T: Into<usize> + Into<i8> {

        // Check that the price of the order is divisible by the lot size and so accepted by the order book
        if !self.is_divisible_by_lot_size(order.price) {
            return Err(Error::LOBError(String::from("Order price is not divisible by the lot size")))
        }

        match order.side {
            Side::Buy => {
                // If true the buy side will add liquidity and not take from the sell side
                if order.price < self.ask_price {
                    // If true the buy side will add liquidity and shift the bid price
                    if order.price > self.bid_price {

                        // Add blank lots in the list so that the new order is front
                        let no_missing_lots: usize = ((order.price - self.bid_price) / self.lot_size).into();

                        self.bid_price = order.price;

                        for i in 0..no_missing_lots {
                            if i < no_missing_lots - 1 {
                                self.buy_price_points.push_front(VecDeque::new());
                            } else {
                                let mut tmp = VecDeque::new();
                                tmp.push_front(order);
                                self.buy_price_points.push_front(tmp);
                            }
                        }
                    } else {

                    }
                } else {

                }
            },
            Side::Sell => {

            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn setup_order_book<T>() -> OrderBook<T> {

        let lob = OrderBook::new(
            SecuritySymbol::Equity(String::from("Test")),
            0.01 as f64
        );


       lob
    }

    fn test_lob_add_limit_order() {

        // Arrange



        let expected =

        // Act

        // Assert

    }



}