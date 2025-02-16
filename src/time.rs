use std::time::{Instant, Duration};

pub struct Time {

    pub startTime: Instant,
    pub deltaTime: Duration,
    pub prevTime: Duration,

}

impl Time {
    pub fn new() -> Self {
        return Self {
            startTime: Instant::now(),
            deltaTime: Duration::from_micros(0),
            prevTime: Duration::from_millis(0),
        }
    }
}

impl Time {

    pub fn updateDeltaTime(&mut self) {
        let elapsed = self.startTime.elapsed();

        self.deltaTime = elapsed - self.prevTime;
        self.prevTime = elapsed;
    }

}