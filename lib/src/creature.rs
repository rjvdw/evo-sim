//! Represents a creature that lives inside the simulation.

use std::fmt;

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
