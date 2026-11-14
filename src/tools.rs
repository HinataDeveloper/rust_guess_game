pub struct Counter {
    counter: u32,
}

impl Counter {
    #[must_use]
    pub const fn new(start: u32) -> Self {
        Self { counter: start }
    }

    #[must_use]
    pub const fn status(&self) -> u32 {
        self.counter
    }

    pub const fn plus(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    pub const fn minus(&mut self) -> u32 {
        self.counter -= 1;
        self.counter
    }

    #[must_use]
    pub const fn check(&mut self, with: u32) -> bool {
        self.counter == with
    }

    pub const fn reset(&mut self) {
        self.counter = 0;
    }

    pub const fn reset_by(&mut self, by: u32) {
        self.counter = by;
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.counter)
    }
}

impl std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counter")
            .field("counter", &self.counter)
            .finish()
    }
}

#[allow(dead_code)]
pub fn logger(msg: &str) {
    let now = chrono::offset::Local::now();
    println!(" >>> Logger: {now:?} {msg} ");
}
