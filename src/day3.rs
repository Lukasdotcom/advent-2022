// Used to get the data
const DATA: &str = include_str!("../data/day3.txt");
pub fn main() {
    let data: Vec<&str> = DATA.split("\n").collect();
    let mut sum = 0;
    // Goes through every line in the file
    for x in &data {
        let parts = x.split_at(x.len() / 2);
        let part1 = parts.0;
        let part2 = parts.1;
        // Goes through every letter in the first half an checks if it is in part2
        for letter in part1.chars() {
            if part2.contains(letter) {
                // Calculates the score for that letter
                let mut number = letter as i32;
                if number <= 90 {
                    number -= 38;
                } else {
                    number -= 96;
                }
                sum += number;
                break;
            }
        }
    }
    println!("Answer for day 3 part 1 is {}.", sum);
    assert_eq!(sum, 7701);
    let mut sum2 = 0;
    // Goes through every group of 3 lines in the file
    for x in 0..((&data).len() / 3) {
        // Checks which of the letters in the first elf are in all of the other ones
        for letter in data[x * 3].chars() {
            if data[x * 3 + 1].contains(letter) && data[x * 3 + 2].contains(letter) {
                // Calculates the score for that letter
                let mut number = letter as i32;
                if number <= 90 {
                    number -= 38;
                } else {
                    number -= 96;
                }
                sum2 += number;
                break;
            }
        }
    }
    println!("Answer for day 3 part 2 is {}.", sum2);
    assert_eq!(sum2, 2644);
}
