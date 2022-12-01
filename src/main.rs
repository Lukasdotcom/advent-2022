mod day1;
// The day to show (0 means all of them)
const DAY:i32 = 0;
fn main() {
    if DAY == 0 || DAY == 1 {
        day1::main();
    }
}
