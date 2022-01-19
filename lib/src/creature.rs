//! Represents a creature that lives inside the simulation.

use std::fmt;

use rand::Rng;

/// A creature that lives inside the simulation.
#[derive(Debug, Copy, Clone)]
pub struct Creature {
    id: u64,
    energy: u8,
    position: (u32, u32),
}

impl Creature {
    /// Initializes a new creature with a given ID and position.
    pub fn new(id: u64, position: (u32, u32)) -> Creature {
        Creature {
            id,
            energy: 100,
            position,
        }
    }

    /// Walk towards a new location.
    pub fn walk(&mut self, max_row: u32, max_col: u32) {
        let mut rng = rand::thread_rng();
        self.position = match rng.gen_range(0..4) {
            0 if self.position.1 + 1 < max_col => (self.position.0, self.position.1 + 1),
            0 => (self.position.0, self.position.1 - 1),
            1 if self.position.1 > 0 => (self.position.0, self.position.1 - 1),
            1 => (self.position.0, self.position.1 + 1),
            2 if self.position.0 + 1 < max_row => (self.position.0 + 1, self.position.1),
            2 => (self.position.0 - 1, self.position.1),
            3 if self.position.0 > 0 => (self.position.0 - 1, self.position.1),
            _ => (self.position.0 + 1, self.position.1),
        }
    }

    pub fn position(&self) -> (u32, u32) {
        self.position
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Creature #{} is at ({}, {}) has {} energy.",
            self.id, self.position.0, self.position.1, self.energy
        )
    }
}
