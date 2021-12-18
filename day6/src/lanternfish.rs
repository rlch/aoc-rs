pub struct Lanternfish {
    pub timer: u32,
}

impl Lanternfish {
    pub fn new(timer: u32) -> Self {
        Self { timer }
    }

    pub fn spawn() -> Self {
        Self { timer: 9 }
    }

    pub fn end_day(&mut self) {
        self.timer -= 1;
    }

    /// Returns [Some(Lanternfish)] if a new lanternfish should be spawned.
    pub fn start_new_day(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 7;
            Some(Lanternfish::spawn())
        } else {
            None
        }
    }
}
