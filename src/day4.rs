// Used to get the data
const DATA: &str = include_str!("../data/day4.txt");
pub fn main() {
    let data: Vec<&str> = DATA.split("\n").collect();
    let mut overlap = 0;
    let mut complete_overlap = 0;
    // Goes through every line in the file
    for x in data {
        let mut parts = x.split(",");
        let first_part: Vec<&str> = parts.next().expect("Bad Input").split("-").collect();
        let second_part: Vec<&str> = parts.next().expect("Bad Input").split("-").collect();
        let num1_1: i32 = first_part
            .get(0)
            .expect("Bad Input")
            .parse()
            .expect("Not a number");
        let num1_2: i32 = first_part
            .get(1)
            .expect("Bad Input")
            .parse()
            .expect("Not a number");
        let num2_1: i32 = second_part
            .get(0)
            .expect("Bad Input")
            .parse()
            .expect("Not a number");
        let num2_2: i32 = second_part
            .get(1)
            .expect("Bad Input")
            .parse()
            .expect("Not a number");
        if (num1_1 <= num2_1 && num1_2 >= num2_2) || (num1_1 >= num2_1 && num1_2 <= num2_2) {
            complete_overlap += 1;
        }
        if !((num1_2 < num2_1) || (num2_2 < num1_1)) {
            overlap += 1;
        }
    }
    println!("Answer for day 4 part 1 is {}.", complete_overlap);
    assert_eq!(complete_overlap, 477);
    println!("Answer for day 4 part 2 is {}.", overlap);
    assert_eq!(overlap, 830);
}
