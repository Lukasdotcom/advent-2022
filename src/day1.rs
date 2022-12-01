use std::fs;
// The numer of elves for part 2
const NUMBER: usize = 3;
pub fn main() {
    // Used to get the data
    let data =
        fs::read_to_string("data/day1.txt").expect("Should have been able to read day1.txt file");
    let data: Vec<&str> = data.split("\n").collect();
    let mut max = [0; NUMBER];
    let mut total = 0;
    // Goes through every line in the file
    for x in data {
        let num = x.parse::<i32>();
        // Checks if the line is a number
        if num.is_err() {
            // If it is not a number it is put in the right place in the array of the top 3 elves
            let mut num = -1;
            for x in 0..NUMBER {
                if num != -1 {
                    let tempnum = max[x];
                    max[x] = num;
                    num = tempnum
                } else if total > max[x] {
                    num = max[x];
                    max[x] = total;
                }
            }
            total = 0;
        } else {
            // Adds to the total if it is a number
            total += num.expect("");
        }
    }
    println!("Answer for day 1 part 1 is {}.", max[0]);
    let sum: i32 = max.iter().sum();
    println!("Answer for day 1 part 1 with {NUMBER} elves is {sum}.");
}
