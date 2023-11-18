#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cube {
    size: u8,
    sides: [char; 6],
}

impl Cube {
    #[must_use]
    pub const fn new(size: u8, sides: [char; 6]) -> Self {
        Self { size, sides }
    }

    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    #[must_use]
    pub const fn side(&self, index: usize) -> char {
        self.sides[index]
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(20, ['@', '$', '~', '#', ';', '+'])
    }
}
