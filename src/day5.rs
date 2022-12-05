// Used to get the data
const DATA: &str = include_str!("../data/day5.txt");
pub fn main() {
    let data: Vec<&str> = DATA.split("\n\n").collect();
    let mut alignment_data: Vec<&str> = data[0].split("\n").collect(); // Grabs the starting alignment
    alignment_data.reverse();
    let rows = (alignment_data[0].len() - 3) / 4 + 1;
    let mut alignment: Vec<Vec<char>> = [].to_vec(); // Stores the movement for part 1
    let mut alignment2 = alignment.clone(); // Stores the movement for part 2
                                            // Generates for every column a vector
    for _ in 0..rows {
        alignment.push(vec![]);
        alignment2.push(vec![]);
    }
    let moves: Vec<&str> = data[1].split("\n").collect(); // Stores all the moves
                                                          // Goes through every line horizantly in the data
    for line in 1..alignment_data.len() {
        let line = alignment_data[line];
        // Checks each column if it contains something
        for i in 0..rows {
            // Adds the character to the column if needed
            let data = line.chars().nth(4 * i + 1);
            match data {
                Some(x) => {
                    if x != ' ' {
                        alignment[i].push(x);
                        alignment2[i].push(x);
                    }
                }
                None => {}
            }
        }
    }
    for play in moves.iter() {
        // This stores the orders
        let parts: Vec<&str> = play.split(" ").collect();
        let crane_size = parts[1].parse::<usize>().expect("NAN");
        let grab_column = parts[3].parse::<usize>().expect("NAN") - 1;
        let drop_column = parts[5].parse::<usize>().expect("NAN") - 1;
        // Movement for part 1
        for _ in 0..crane_size {
            let pop = alignment[grab_column].pop().expect("Should not be empty");
            alignment[drop_column].push(pop);
        }
        // Movement for part 2
        let mut movements: Vec<char> = vec![]; // Stores what the crane carries
                                               // Adds everything to the crane
        for _ in 0..crane_size {
            let pop = alignment2[grab_column].pop().expect("Should not be empty");
            movements.push(pop);
        }
        // Drops off the crane
        while movements.len() > 0 {
            let pop = movements.pop().expect("Should not be empty");
            alignment2[drop_column].push(pop);
        }
    }
    // Generates the answers
    let mut answer = String::from("");
    for x in alignment {
        answer.push(x[x.len() - 1]);
    }
    let mut answer2 = String::from("");
    for x in alignment2 {
        answer2.push(x[x.len() - 1]);
    }
    println!("Answer for day 5 part 1 is {}.", answer);
    assert_eq!(answer, "CWMTGHBDW");
    println!("Answer for day 5 part 2 is {}.", answer2);
    assert_eq!(answer2, "SSCGWJCRB");
}
