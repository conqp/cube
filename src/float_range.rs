#[derive(Debug)]
pub struct FloatRange {
    value: f64,
    end: f64,
    step: f64,
}

impl FloatRange {
    #[must_use]
    pub const fn new(start: f64, end: f64, step: f64) -> Self {
        Self {
            value: start,
            end,
            step,
        }
    }
}

impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < self.end {
            let value = self.value;
            self.value += self.step;
            Some(value)
        } else {
            None
        }
    }
}
