use std::time::Instant;


pub struct Timer {
    pub delta_time: f64,
    time: Instant,
    pub time_last_frame: f64,
    frames: usize
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            delta_time: 0.0,
            time_last_frame: 0.0,
            frames: 0
        }
    }

    pub fn fps(&self) -> f64 {
        // if self.frames < 1 || self.time_last_frame < 1 {
        //     0
        // }
        // else {
        //     (self.frames/self.time_last_frame as usize) as u32
        // }
        1.0/self.delta_time
    }

    pub fn update(&mut self) {
        self.delta_time = self.time.elapsed().as_secs_f64()-self.time_last_frame;
        self.time_last_frame = self.time.elapsed().as_secs_f64();
        self.frames += 1;
        if  self.frames > 2_000_000 {
            self.frames = 0;
        }
    }
}