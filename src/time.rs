use std::time::Instant;


pub struct Timer {
    pub delta_time: f32,
    time: Instant,
    pub time_last_frame: f32,
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

    pub fn get_fps(&self) -> u32 {
        if self.frames < 1 || self.time_last_frame < 1.0  {
            0
        }
        else {
            (self.frames/self.time_last_frame as usize) as u32
        }
    }

    pub fn update(&mut self) {
        self.delta_time = self.time.elapsed().as_secs_f32()-self.time_last_frame;
        self.time_last_frame = self.time.elapsed().as_secs_f32();
        self.frames += 1;
        if  self.frames > 2_000_000 {
            self.frames = 0;
        }
    }
}