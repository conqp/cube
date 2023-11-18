const DEFAULT_SIZE: u8 = 20;
const DEFAULT_SIDES: [&str; 6] = [
    "\x1b[31m@\x1b[0m",
    "\x1b[32m$\x1b[0m",
    "\x1b[33m~\x1b[0m",
    "\x1b[34m#\x1b[0m",
    "\x1b[35m;\x1b[0m",
    "\x1b[36m+\x1b[0m",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    size: u8,
    sides: [&'static str; 6],
}

impl Cube {
    #[must_use]
    pub const fn new(size: u8, sides: [&'static str; 6]) -> Self {
        Self { size, sides }
    }

    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    #[must_use]
    pub const fn side(&self, index: usize) -> &'static str {
        self.sides[index]
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(DEFAULT_SIZE, DEFAULT_SIDES)
    }
}
