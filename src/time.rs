use std::time::Instant;


pub struct Timer {
    pub delta_time: f64,
    time: Instant,
    pub time_last_frame: f64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            delta_time: 0.0,
            time_last_frame: 0.0,
        }
    }

    pub fn fps(&self) -> f64 {
        1.0/self.delta_time
    }

    pub fn update(&mut self) {
        self.delta_time = self.time.elapsed().as_secs_f64();
        self.time = Instant::now();
    }
}