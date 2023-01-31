use std::time::Instant;

const NS_PER_FPS: f64 = 1_000_000_000.0 / 60.0;

// calculating delta time for later use.
pub struct Time {
    pub delta: f64,
    pub frames: u32,
    pub updates: u32,
    last_time: Instant,
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta += (now - self.last_time).as_nanos() as f64 / NS_PER_FPS;
        self.last_time = now;
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            delta: Default::default(),
            frames: Default::default(),
            updates: Default::default(),
            last_time: Instant::now(),
        }
    }
}
