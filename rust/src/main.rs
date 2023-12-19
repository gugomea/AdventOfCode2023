use std::time::Instant;

use rust::day15::solve;

fn main() { 
    let instant = Instant::now();
    solve();
    let elapsed = instant.elapsed();
    println!("=============");
    println!("{:?}", elapsed);
    println!("=============");
}
