use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CubeTimer {
    starttime: Option<Instant>,
    on: bool,
    lasttime: Duration,
}

impl CubeTimer {
    pub fn default() -> Self {
        Self {
            starttime: None,
            on: false,
            lasttime: Duration::new(0, 0),
        }
    }

    pub fn space_press(&mut self) {
        match self.on {
            false => self.timer_on(),
            true => self.timer_off(),
        }
    }

    fn timer_on(&mut self) {
        self.on = true;
        self.starttime = Some(Instant::now());
    }

    fn timer_off(&mut self) {
        self.on = false;
        self.lasttime = self.elapsed();
        self.starttime = None;
    }

    fn elapsed(&self) -> Duration {
        match self.starttime {
            Some(v) => v.elapsed(),
            None => Duration::new(0, 0),
        }
    }

    pub fn text(&self) -> String {
        match self.starttime {
            Some(v) => format!("{:.1}", v.elapsed().as_secs_f32()),
            None => format!("{:.3}", self.lasttime.as_secs_f32()),
        }
    }
}