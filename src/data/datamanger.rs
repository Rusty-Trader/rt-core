use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std:: sync::mpsc::{channel, Receiver, Sender};
use std::default::Default;
use std::time::Duration;
use crate::data::fx_manager::FXManager;

use crate::DataNumberType;
use crate::rtengine::{BackTester, RunMode};
use crate::security::{SecuritySymbol, SecurityType};
use crate::time::TimeSync;
use crate::utils::Merge;

use super::DataPoint;
use super::datafeed::{DataFeed, DataFeedBuilder};
use super::error::DataError;
use super::slice::Slice;



pub struct DataManager<T> where
    T: Clone + DataNumberType {

    buffer: VecDeque<DataPoint<T>>, // TODO: Deprecate

    securities: HashMap<SecuritySymbol, String>,

    fxmanager: FXManager<T>,

    feeds: HashMap<String, Box<dyn DataFeed<NumberType = T>>>,

    sender: Sender<DataPoint<T>>, // TODO: Needs to be a HashMap with key per feed

    receiver: Receiver<DataPoint<T>>,

    time: TimeSync,

    mode: RunMode,

    fill_sender: Option<Sender<DataPoint<T>>>


}


impl<T> DataManager<T> where
    T: Clone + DataNumberType {

    pub fn new(time: TimeSync, mode: RunMode) -> DataManager<T> {

        let (sender, receiver) = channel();

        DataManager{
            buffer: VecDeque::new(),
            securities: HashMap::new(),
            fxmanager: FXManager::new(),
            feeds: HashMap::new(),
            sender: sender,
            receiver: receiver,
            time,
            mode,
            fill_sender: None
        }
    }

    pub fn connect(&mut self) -> Result<(), DataError> {
        for (_, feed) in &mut self.feeds {
            feed.connect(self.sender.clone(), self.mode)?
        };

        Ok(())
    }

    pub fn add_feed<D: DataFeedBuilder<NumberType = T>>(&mut self, datafeed: D)
        where <D as DataFeedBuilder>::Output: 'static + DataFeed<NumberType = T> {

        self.feeds.insert("test".to_string(), Box::new(datafeed
            .with_mode(self.mode)
            .with_time(self.time.get_ptr())
            .with_sender(self.sender.clone())
            .build().unwrap())
        );
    }

    pub fn get_slice(&mut self) -> Slice<T> where T: DataNumberType {
        // TODO: Add FillFwd
        let mut wait_time = Duration::new(0, 1000000);

        let mut slice: Slice<T> = Slice::new(self.time.get_time());

        let interval: (i64, i64) = (self.time.period_start(), self.time.get_time());

        for val in self.receiver.try_iter() {
            if val.time > interval.0 && val.time <= interval.1 {
                // slice.merge(val.clone().into())
                slice.add_datapoint(val.clone())
            }


            if let (SecuritySymbol::FX(base, foreign), Some(x)) = (val.get_symbol(), val.data.get_spot()) {
                self.fxmanager.update(base, foreign, x)
            }

            // TODO: Move to next cycle
            if let Some(sender) = &self.fill_sender {
                sender.send(val);
            }
        }

        slice
        
    }

    pub fn with_fill_sender(&mut self) -> Receiver<DataPoint<T>> {
        let (sender, receiver) = channel();

        self.fill_sender = Some(sender);

        receiver
    }

    pub fn is_finished(&self) -> bool {

        let mut tmp: bool = true;

        for (_, feed) in &self.feeds {
            tmp = tmp && feed.is_finished()
        }

        tmp
        
    }

    pub fn feeds_send_backtest(&mut self) -> Result<(), DataError> {
        
        for (_, value) in &mut self.feeds {
            value.send_backtest()?
        }

        Ok(())
    }

    pub fn symbol_exists(&self, symbol: SecuritySymbol) -> bool {
        self.securities.contains_key(&symbol)
    }

}

// impl<T> BackTester for DataManager<T> where T: Clone {

//     fn next_cycle(&mut self) -> Result<(), crate::error::Error> {


//         if let Some(sender) = &self.fill_sender {
//             sender.send(val.into());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
