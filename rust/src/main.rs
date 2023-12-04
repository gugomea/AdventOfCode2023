use std::error::Error;
use rust::day2::solve;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    solve();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
