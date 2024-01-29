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

    // fn limit_order_buy(&mut self, order: &LOBOrder) -> Result<(), Error> {
    //     let mut order_vol = order.volume;
    //
    //     let mut order_matches: Vec<LOBOrder> = Vec::new();
    //
    //     for i in 0..=self.price_to_index(order.price, Side::Buy) {
    //         // Access the index add the ask price and move up
    //         if let Some(matched_index) = self.sell_price_points.get_mut(i) {
    //
    //             for i in 0..=matched_index.len() {
    //
    //                 if let Some(matched_order) = matched_index.get_mut(i) {
    //                     if order_vol < matched_order.volume {
    //                         matched_order.volume -= order_vol;
    //                         order_matches.push(order.to_owned());
    //                     } else {
    //                         order_vol -= matched_order.volume;
    //                         order_matches.push(order.to_owned());
    //                     }
    //                 }
    //             }
    //
    //         }
    //     }
    //
    //     Ok(())
    //
    // }

    fn limit_order_buy(&mut self, order: &LOBOrder) -> Result<(), Error> {

        // If true the buy side will add liquidity and not take from the sell side
        if (order.price < self.ask_price) && (!self.buy_price_points.is_empty()) {
            // If true the buy side will add liquidity and shift the bid price
            if order.price > self.bid_price {

                // Add blank lots in the list so that the new order is front
                let no_missing_lots = ((order.price - self.bid_price) / self.lot_size) as u64;

                self.bid_price = order.price;

                for i in 0..no_missing_lots {
                    if i < no_missing_lots - 1 {
                        self.buy_price_points.push_front(VecDeque::new());
                    } else {
                        let mut tmp: VecDeque<LOBOrder> = VecDeque::new();
                        tmp.push_front(order.clone());
                        self.buy_price_points.push_front(tmp);
                    }
                }
            } else {
                let lot_position = ((self.bid_price - order.price) / self.lot_size) as u64;

                self.buy_price_points[lot_position as usize].push_back(order.clone());

            }
        } else if (self.buy_price_points.is_empty()) || ((order.price >= self.ask_price) && self.sell_price_points.is_empty()) {

            self.bid_price = order.price;

            if self.sell_price_points.is_empty() {
                self.ask_price = self.bid_price + self.lot_size;
            }

            let mut tmp = VecDeque::new();
            tmp.push_front(order.clone());
            self.buy_price_points.push_front(tmp);


        } else {
            self.market_order(order.side, order.volume)?
        }

        Ok(())
    }

    fn limit_order_sell(&mut self, order: &LOBOrder) -> Result<(), Error> {

        if (order.price > self.bid_price) && (!self.sell_price_points.is_empty()) {

            if order.price < self.ask_price {

                // Add blank lots in the list so that the new order is front
                let no_missing_lots = ((self.ask_price - order.price) / self.lot_size) as u64;

                self.ask_price = order.price;

                for i in 0..no_missing_lots {
                    if i < no_missing_lots - 1 {
                        self.sell_price_points.push_front(VecDeque::new());
                    } else {
                        let mut tmp = VecDeque::new();
                        tmp.push_front(order.clone());
                        self.sell_price_points.push_front(tmp);
                    }
                }
            } else {

            }
        } else if self.sell_price_points.is_empty() {

            self.ask_price = order.price;

            let mut tmp = VecDeque::new();
            tmp.push_front(order.clone());
            self.sell_price_points.push_front(tmp);


        } else {
            self.market_order(order.side, order.volume)?
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
                self.limit_order_buy(&order)?
            },
            Side::Sell => {
                self.limit_order_sell(&order)?
            }
        }
        Ok(())
    }

    pub fn market_order(&mut self, side: Side, volume: f64) -> Result<() , Error> {

        let mut current_volume = volume;

        match side {
            Side::Buy => {
                if !self.sell_price_points.is_empty() {
                    while current_volume > 0.0 {
                        let front_price_point = &mut self.sell_price_points[0][0];
                        if front_price_point.volume <= current_volume {
                            current_volume -= front_price_point.volume
                        } else {
                            front_price_point.volume -= current_volume;
                            current_volume = 0.0;
                        }

                    }
                }
            },
            Side::Sell => {
                if !self.buy_price_points.is_empty() {
                    while current_volume > 0.0 {
                        let front_price_point = &mut self.buy_price_points[0][0];
                        if front_price_point.volume <= current_volume {
                            current_volume -= front_price_point.volume
                        } else {
                            front_price_point.volume -= current_volume;
                            current_volume = 0.0;
                        }

                    }
                }


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

    fn setup_order_book_with_spread() -> OrderBook {

        let mut lob = OrderBook::new(
            SecuritySymbol::Equity(String::from("Test")),
            0.01
        );

        let buy_order = LOBOrder::new(
            Side::Buy,
            10.0,
            1.1,
            1001,
            1
        );

        let sell_order_1 = LOBOrder::new(
            Side::Sell,
            8.0,
            1.3,
            1002,
            2
        );

        let sell_order_2 = LOBOrder::new(
            Side:: Sell,
            1.0,
            1.3,
            1001,
            3,
        );

        let sell_order_3 = LOBOrder::new(
            Side::Sell,
            5.0,
            1.25,
            1001,
            4
        );

        _ = lob.add_limit_order(buy_order);
        _ = lob.add_limit_order(sell_order_1);
        _ = lob.add_limit_order(sell_order_2);
        _ = lob.add_limit_order(sell_order_3);

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
        let mut lob = setup_empty_order_book();

        let order = LOBOrder::new(
            Side::Sell,
            10.0,
            1.1,
            1001,
            1
        );

        let expected_order = order;
        let expected_ask_price = 1.1;

        // Act
        _ = lob.add_limit_order(order);

        let result_order = lob.sell_price_points[0][0];
        let result_ask_price = lob.ask_price;

        // Assert
        assert_eq!(result_order, expected_order);
        assert_eq!(result_ask_price, expected_ask_price)

    }


    #[test]
    fn test_lob_add_buy_limit_in_spread() {

        // Arrange
        let mut lob = setup_order_book_with_spread();

        let order = LOBOrder::new(
            Side::Buy,
            9.0,
            1.2,
            1003,
            3
        );

        let expected_order = order;
        let expected_bid_price = 1.2;


        // Act
        _ = lob.add_limit_order(order);

        let result_order = lob.buy_price_points[0][0];
        let result_bid_price = lob.bid_price;

        // Assert
        assert_eq!(result_order, expected_order);
        assert_eq!(result_bid_price, expected_bid_price)

    }

    #[test]
    fn test_lob_buy_limit_order_take_liquidity() {

        // Arrange
        let mut lob = setup_order_book_with_spread();

        let order = LOBOrder::new(
            Side::Sell,
            9.0,
            1.1,
            1003,
            2
        );

        let expected_order = LOBOrder::new(
            Side::Buy,
            1.0,
            1.1,
            1001,
            1
        );

        let expected_bid_price = 1.1;

        // Act
        _ = lob.add_limit_order(order);

        let result_order = lob.buy_price_points[0][0];
        let result_bid_price = lob.bid_price;

        // Assert
        assert_eq!(result_order, expected_order);
        assert_eq!(result_bid_price, expected_bid_price)
    }


    // TODO: Test to check that bid and ask prices are updated with market order

    #[test]
    fn test_lob_multiple_buy_limits() {

        // Arrange
        let mut lob = setup_empty_order_book();

        let buy_order_1 = LOBOrder::new(
            Side::Buy,
            1.0,
            1.09,
            1001,
            1
        );

        let buy_order_2 = LOBOrder::new(
            Side::Buy,
            5.0,
            1.09,
            1002,
            2
        );

        let buy_order_3 = LOBOrder::new(
            Side::Buy,
            4.0,
            1.1,
            1001,
            4
        );

        let expected_order_1 = buy_order_1;
        let expected_order_2 = buy_order_2;
        let expected_order_3 = buy_order_3;
        let expected_bid_price = 1.09;


        // Act
        _ = lob.add_limit_order(buy_order_1);
        _ = lob.add_limit_order(buy_order_2);
        _ = lob.add_limit_order(buy_order_3);

        let result_1 = lob.buy_price_points[0][0];
        let result_2 = lob.buy_price_points[0][1];
        let result_3 = lob.buy_price_points[1][0];
        let result_bid_price = lob.bid_price;

        // Assert
        assert_eq!(result_bid_price, expected_bid_price);
        assert_eq!(result_1, expected_order_1);
        assert_eq!(result_2, expected_order_2);
        assert_eq!(result_3, expected_order_3);

    }

    #[test]
    fn test_lob_sell_limit_order_take_liquidity() {

        // Arrange
        let mut lob = setup_order_book_with_spread();

        let order = LOBOrder::new(
            Side::Buy,
            9.0,
            1.1,
            1003,
            2
        );

        let expected_order = LOBOrder::new(
            Side::Sell,
            5.0,
            1.25,
            1001,
            4
        );

        let expected_ask_price = 1.25;

        // Act
        _ = lob.add_limit_order(order);

        let result_order = lob.sell_price_points[0][0];
        let result_bid_price = lob.ask_price;

        // Assert
        assert_eq!(result_order, expected_order);
        assert_eq!(result_bid_price, expected_ask_price)

    }
}