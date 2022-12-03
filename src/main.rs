// The day to show (0 means all of them)
const DAY:i32 = 0;
mod day1;
mod day2;
mod day3;
fn main() {
    if DAY == 0 || DAY == 1 {
        day1::main();
    }
    if DAY == 0 || DAY == 2 {
        day2::main();
    }
    if DAY == 0 || DAY == 3 {
        day3::main();
    }
}
