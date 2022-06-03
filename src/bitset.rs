use std::fmt;

/// A bitarray of length 8
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitset (u8);

impl Bitset {
    pub fn new() -> Bitset {
        Bitset(0)
    }

    pub fn get(&self, index: usize) -> bool {
        debug_assert!(index <= 7);
        self.0 & (1 << index) > 0
    }

    pub fn set(&mut self, index: usize, val: bool) {
        let mask = 1 << index;
        if val {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }

    /// Create a bitset where the first n bits are marked false
    pub fn first_n(n: usize) -> Bitset {
        match n {
            0 => Bitset(0b11111111),
            1 => Bitset(0b11111110),
            2 => Bitset(0b11111100),
            3 => Bitset(0b11111000),
            4 => Bitset(0b11110000),
            5 => Bitset(0b11100000),
            6 => Bitset(0b11000000),
            7 => Bitset(0b10000000),
            8 => Bitset(0b00000000),
            _ => panic!("Invalid size requested for bitset"),
        }
    }

    /// Test if any bits are set to true
    pub fn any(self) -> bool {
        self.0 != 0
    }

    /// Tests if all bits are false
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Tests if all bits are true
    pub fn is_full(self) -> bool {
        self.0 == 0xff
    }
}

impl fmt::Debug for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set({:08b})", self.0)
    }
}

