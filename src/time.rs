use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use crate::data::Resolution;
use crate::rtengine::RunMode;

#[derive(Debug, Clone)]
pub struct TimeSync {

    time: Arc<AtomicI64>,

    resolution: Resolution,

    is_quantized: bool

}

impl TimeSync {

    pub fn new(time: i64, resolution: Resolution) -> Self {
        Self {
            time: Arc::new(AtomicI64::new(time)),
            resolution,
            is_quantized: false
        }
    }

    pub fn from_atomic(time: Arc<AtomicI64>, resolution: Resolution) -> Self {
        Self {
            time,
            resolution,
            is_quantized: false
        }
    }

    pub fn get_time(&self) -> i64 {
        self.time.load(Ordering::Relaxed)
    }

    pub fn get_ptr(&self) -> Arc<AtomicI64> {
        self.time.clone()
    }

    pub fn update_time(&mut self, mode: RunMode) {

        match mode {
            RunMode::LiveTrade => (),
            RunMode::PaperTrade => (),
            RunMode::BackTest => self.update_time_backtest(),
            RunMode::UnitTest => ()
        }

    }

    pub fn period_start(&self) -> i64 {
        self.get_time() - self.resolution as i64
    }

    fn update_time_backtest(&mut self) {

        if !self.is_quantized {
            self.quantize_time()
        }

        self.time.fetch_add(self.resolution as i64, Ordering::Relaxed);

    }

    fn quantize_time(&mut self) {
        self.time.fetch_sub(self.time.load(Ordering::Relaxed) % self.resolution as i64, Ordering::Relaxed);
        self.is_quantized = true
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_time_backtest() {

        // Arrange
        let mut time = TimeSync::new(1050, Resolution::Second);

        let expected = 2000;

        // Act
        time.update_time(RunMode::BackTest);

        let result = time.get_time();

        // Assert
        assert_eq!(result, expected)

    }


}
