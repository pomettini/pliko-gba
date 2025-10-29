use agb::timer::{Divider, Timer};

pub struct Countdown<'a> {
    seconds: usize,
    seconds_left: usize,
    t2: &'a mut Timer,
    t3: &'a mut Timer,
    last_ticks: u32,
    acc: u32,
}

impl<'a> Countdown<'a> {
    pub fn new(seconds: usize, t2: &'a mut Timer, t3: &'a mut Timer) -> Self {
        t2.set_divider(Divider::Divider1024).set_enabled(true);
        t3.set_cascade(true).set_enabled(true);

        Self {
            seconds,
            seconds_left: 0,
            t2,
            t3,
            last_ticks: 0,
            acc: 0,
        }
    }

    pub const fn seconds_left(&self) -> usize {
        self.seconds_left
    }

    pub fn reset(&mut self) {
        self.last_ticks = 0;
        self.seconds_left = self.seconds;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.t2.set_enabled(enabled);
        self.t3.set_enabled(enabled);
    }

    pub fn update(&mut self) {
        let ticks = ((self.t3.value() as u32) << 16) | (self.t2.value() as u32);
        let delta = ticks.wrapping_sub(self.last_ticks);
        self.last_ticks = ticks;
        self.acc = self.acc.wrapping_add(delta);
        if self.acc >= 0x4000 {
            self.acc -= 0x4000;
            self.seconds_left -= 1;
        }
    }
}
