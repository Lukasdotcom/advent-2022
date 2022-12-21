// Used to get the data
const DATA: &str = include_str!("../data/day6.txt");
pub fn main() {
    let data: Vec<char> = DATA.chars().collect();
    let mut character_list: [&char; 3] = [&'1'; 3]; // Stores the last 3 characters
    let mut dedup = 4; // Stores how long it will take to finish
    let mut part1 = 0; // Stores the counter for part1
    for character in &data {
        part1 += 1;
        // Checks if the new character already exists
        for x in 0..character_list.len() {
            if character_list[x] == character {
                let new_dedup = x + 1;
                if dedup < new_dedup {
                    dedup = new_dedup;
                }
            }
            // Moves the character down one unles it is the last then the new character is added
            if x != character_list.len() - 1 {
                character_list[x] = character_list[x + 1];
            } else {
                character_list[x] = character;
            }
        }
        // Checks if this is a completly unique string
        if dedup == 0 {
            break;
        }
        dedup -= 1;
    }
    let mut character_list: [char; 13] = ['1'; 13]; // Stores the last 3 characters
    let mut dedup = 14; // Stores how long it will take to finish
    let mut part2 = 0; // Stores the counter for part1
    for character in data {
        part2 += 1;
        // Checks if the new character already exists
        for x in 0..character_list.len() {
            if character_list[x] == character {
                let new_dedup = x + 1;
                if dedup < new_dedup {
                    dedup = new_dedup;
                }
            }
            // Moves the character down one unles it is the last then the new character is added
            if x != character_list.len() - 1 {
                character_list[x] = character_list[x + 1];
            } else {
                character_list[x] = character;
            }
        }
        // Checks if this is a completly unique string
        if dedup == 0 {
            break;
        }
        dedup -= 1;
    }
    println!("Answer for day 6 part 1 is {}.", part1);
    assert_eq!(part1, 1892);
    println!("Answer for day 6 part 2 is {}.", part2);
    assert_eq!(part2, 2313);
}