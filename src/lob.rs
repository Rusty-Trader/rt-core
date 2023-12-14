use std::collections::VecDeque;
use crate::broker::orders::Side;
use crate::DataNumberType;
use crate::error::Error;
use crate::security::SecuritySymbol;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LOBOrder {
    side: Side,
    volume: f64,
    price: f64,
    trader: i64,
    order_id: i64
}

impl LOBOrder {

    pub fn new(side: Side, volume: f64, price: f64, trader: i64, order_id: i64) -> Self {
        Self {
            side,
            volume,
            price,
            trader,
            order_id
        }
    }
}


pub struct OrderBook {

    symbol: SecuritySymbol,
    buy_price_points: VecDeque<VecDeque<LOBOrder>>,
    sell_price_points: VecDeque<VecDeque<LOBOrder>>,
    bid_price: f64,
    ask_price: f64,
    lot_size: f64
}

impl OrderBook {

    pub fn new(symbol: SecuritySymbol, lot_size: f64) -> Self {
        Self {
            symbol,
            buy_price_points: VecDeque::new(),
            sell_price_points: VecDeque::new(),
            bid_price: 0.0,
            ask_price: 0.0,
            lot_size
        }
    }


    fn price_to_index(&self, price: f64, side: Side) -> usize {
        match side {
            Side::Buy => {
                ((self.bid_price - price) / self.lot_size) as usize
            },
            Side::Sell => {
                ((price - self.ask_price) / self.lot_size) as usize
            }
        }
    }

    fn is_divisible_by_lot_size(&self, price: f64) -> bool {
        (price % self.lot_size) as u64 == 0
    }

    fn order_match_info(&self, order: &LOBOrder) where {
        // TODO: Add some logging
    }

    fn limit_order_buy(&mut self, order: &LOBOrder) -> Result<(), Error> {
        let mut order_vol = order.volume;

        let mut order_matches: Vec<LOBOrder> = Vec::new();

        for i in 0..=self.price_to_index(order.price, Side::Buy) {
            // Access the index add the ask price and move up
            if let Some(matched_index) = self.sell_price_points.get_mut(i) {

                for i in 0..=matched_index.len() {

                    if let Some(matched_order) = matched_index.get_mut(i) {
                        if order_vol < matched_order.volume {
                            matched_order.volume -= order_vol;
                            order_matches.push(order.to_owned());
                        } else {
                            order_vol -= matched_order.volume;
                            order_matches.push(order.to_owned());
                        }
                    }
                }

            }
        }

        Ok(())

    }

    pub fn add_limit_order(&mut self, order: LOBOrder) -> Result<(), Error> {

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
                        let no_missing_lots = ((order.price - self.bid_price) / self.lot_size) as u64;

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
                } else if self.buy_price_points.is_empty() {

                    self.bid_price = order.price;

                    let mut tmp = VecDeque::new();
                    tmp.push_front(order);
                    self.buy_price_points.push_front(tmp);


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

    fn setup_empty_order_book() -> OrderBook {

        let mut lob = OrderBook::new(
            SecuritySymbol::Equity(String::from("Test")),
            0.01
        );

        lob

    }

    #[test]
    fn test_lob_add_limit_order() {

        // Arrange
        let mut lob = setup_empty_order_book();

        let order = LOBOrder::new(
            Side::Buy,
            10.0,
            1.1,
            1001,
            1
        );

        let expected_order = order;
        let expected_bid_price = 1.1;

        // Act
        _ = lob.add_limit_order(order);

        let result_order = lob.buy_price_points[0][0];
        let result_bid_price = lob.bid_price;

        // Assert
        assert_eq!(result_order, expected_order);
        assert_eq!(result_bid_price, expected_bid_price)

    }

    #[test]
    fn test_lob_sell_limit_order() {

        // Arrange

        // Act

        // Assert
        assert_eq!(0, 1)
    }


    #[test]
    fn test_lob_add_buy_limit_in_spread() {

        // Arrange

        // Act

        // Assert
        assert_eq!(0, 1)
    }

    #[test]
    fn test_lob_buy_limit_order_take_liquidity() {

        // Arrange

        // Act

        // Assert

        assert_eq!(0, 1)
    }



}