use std::time::{Instant, Duration};

pub struct Time {

    pub startTime: Instant,
    deltaTime: Duration,
    pub prevTime: Duration,

    pub fixedDeltaTime: Duration,
    fixedAccumulator: Duration,
    fixedUpdateCount: u32

}

impl Time {
    pub fn new() -> Self {
        return Self {
            startTime: Instant::now(),
            deltaTime: Duration::from_micros(0),
            prevTime: Duration::from_millis(0),
            
            fixedDeltaTime: Duration::from_micros(16670),
            fixedAccumulator: Duration::from_micros(0),
            fixedUpdateCount: 1,
        }
    }
}

impl Time {
    pub(super) fn init(&mut self) {
        self.startTime = Instant::now();
        self.prevTime = Duration::from_millis(0);
        self.fixedAccumulator = Duration::from_millis(0);
    }

     pub(super) fn updateDeltaTime(&mut self) {
        let elapsed = self.startTime.elapsed();

        self.deltaTime = elapsed - self.prevTime;
        self.prevTime = elapsed;


        // Updating fixed accumulator to track fixed update
        self.fixedAccumulator += self.deltaTime;    
        
        while self.fixedAccumulator >= self.fixedDeltaTime {
            
            self.fixedUpdateCount += 1;
            self.fixedAccumulator -= self.fixedDeltaTime;

        }
    }

    pub(super) fn updateFixed(&mut self) -> bool {
        let count = self.fixedUpdateCount;

        if count == 0 {
            return false;
        }

        self.fixedUpdateCount -= 1;

        return count > 0;
    } 


}