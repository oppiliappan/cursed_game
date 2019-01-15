mod universe;

use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello, world!");
    let mut myuniverse = universe::Universe::new(10, 10);
    for _ in 0..100 {
        println!("{}", myuniverse);
        thread::sleep(Duration::from_millis(1000));
        myuniverse.tick();
    }
}
