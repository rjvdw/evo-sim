//! Runs the simulation and records the results to a file.

use std::thread;
use std::time::Duration;

use rdcl_evo_sim_lib::field::Field;

fn main() {
    let mut field = Field::init(125, 35, 10);

    clear();
    println!("{field}");
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(100));
        field.next();
        clear();
        println!("{field}");
    }
}

fn clear() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
