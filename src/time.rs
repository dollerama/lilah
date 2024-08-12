use std::time::Instant;


pub struct Timer {
    pub delta_time: f64,
    pub smooth_delta_time: f64,
    total_delta_time: f64,
    time: Instant,
    pub time_last_frame: f64,
    frames: i32
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            delta_time: 0.0,
            time_last_frame: 0.0,
            smooth_delta_time: 0.0,
            total_delta_time: 0.0,
            frames: 0,
        }
    }

    pub fn fps(&self) -> f64 {
        1.0/self.delta_time
    }

    pub fn start(&mut self) {
        self.time = Instant::now();
    }

    pub fn capture(&mut self) {
        self.delta_time = self.time.elapsed().as_secs_f64();
        self.frames += 1;
        self.total_delta_time += self.delta_time;
        self.smooth_delta_time = self.total_delta_time/self.frames as f64;
    }

    pub fn delta_time_scaled(&self) -> f64 {
        self.delta_time.clamp(0.0, 0.16)
    }
}