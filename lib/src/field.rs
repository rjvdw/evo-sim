//! The field in which all the creatures live.

use std::collections::HashMap;
use std::fmt;

use rand::Rng;

use crate::creature::Creature;

/// A field in which creatures live.
#[derive(Debug)]
pub struct Field {
    creatures: Vec<Box<Creature>>,
    locations: HashMap<(u32, u32), Box<Creature>>,
    width: u32,
    height: u32,
}

impl Field {
    /// Create an initial field.
    pub fn init(width: u32, height: u32, nr_creatures: u64) -> Field {
        let mut rng = rand::thread_rng();
        let mut creatures = vec![];
        let mut locations = HashMap::new();
        for i in 0..nr_creatures {
            let mut row = rng.gen_range(0..height);
            let mut col = rng.gen_range(0..width);

            while locations.contains_key(&(row, col)) {
                row = rng.gen_range(0..height);
                col = rng.gen_range(0..width);
            }

            let creature = Creature::new(i, (row, col));
            creatures.push(Box::new(creature));
            locations.insert((row, col), Box::new(creature));
        }
        Field {
            creatures,
            locations,
            width,
            height,
        }
    }

    /// Return the creature at a given location, if there is a creature at that location.
    pub fn at(&self, row: u32, col: u32) -> Option<Creature> {
        self.locations.get(&(row, col)).map(|boxed| **boxed)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.at(row, col) {
                    Some(_) => write!(f, "&")?,
                    None => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
