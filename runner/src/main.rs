//! Runs the simulation and records the results to a file.

use rdcl_evo_sim_lib::creature::Creature;

fn main() {
    let creature = Creature::new(1, (0, 0));
    println!("{creature}")
}
