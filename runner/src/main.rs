//! Runs the simulation and records the results to a file.

use rdcl_evo_sim_lib::field::Field;

fn main() {
    let field = Field::init(100, 100, 10);
    println!("{field}");
}
