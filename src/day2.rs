// Used to get the data
const DATA: &str = include_str!("../data/day2.txt");
const SCORES: [[i32; 3]; 3] = [
    [3, 6, 0],
    [0, 3, 6],
    [6, 0, 3],
];
const SCORES2: [[i32; 3]; 3] = [
    [3, 1, 2],
    [1, 2, 3],
    [2, 3, 1],
];
pub fn main() {
    let data: Vec<&str> = DATA.split("\n").collect();
    let mut score = 0;
    let mut score2 = 0;
    // Goes through every line in the file
    for x in data {
        let opponent = (x.chars().nth(0).expect("Bad line") as i32) - 64;
        let user = (x.chars().nth(2).expect("Bad line") as i32) - 87;
        score += user;
        // Gives 3 points for tie, 6 for win, and 0 for tie
        score += SCORES[(opponent - 1) as usize][(user - 1) as usize];
        // Gives the points for winning and loosing for part 2
        score2 += (user - 1) * 3;
        // Gives the points for the type part 2
        score2 += SCORES2[(opponent - 1) as usize][(user - 1) as usize];
    }
    println!("Answer for day 2 part 1 is {}.", score);
    assert_eq!(score, 15523);
    println!("Answer for day 2 part 2 is {}.", score2);
    assert_eq!(score2, 15702);
}