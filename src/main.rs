use std::time::Instant;
// The day to show (0 means all of them)
const DAY: i32 = 0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
fn main() {
    let now = Instant::now();
    if DAY == 0 || DAY == 1 {
        day1::main();
    }
    if DAY == 0 || DAY == 2 {
        day2::main();
    }
    if DAY == 0 || DAY == 3 {
        day3::main();
    }
    if DAY == 0 || DAY == 4 {
        day4::main();
    }
    if DAY == 0 || DAY == 5 {
        day5::main();
    }
    if DAY == 0 || DAY == 6 {
        day6::main();
    }
    if DAY == 0 || DAY == 7 {
        day7::main();
    }
    if DAY == 0 || DAY == 8 {
        day8::main();
    }
    if DAY == 0 || DAY == 9 {
        day9::main();
    }
    let elapsed_time = now.elapsed().as_micros();
    if DAY == 0 {
        println!("All the days took {elapsed_time} µs")
    } else {
        println!("Day {DAY} took {elapsed_time} µs")
    }
}
