// Used to get the data
const DATA: &str = include_str!("../data/day9.txt");
pub fn main() {
    let steps = DATA.split("\n");
    // Stores the historical positions
    let mut positions: Vec<Vec<[i32; 2]>> = vec![vec![[1, 1]]]; // Stores every coordinate that has been visited by the start and end of tail
    let mut part1 = 1;
    let mut part2 = 1;
    let mut head = [0, 0];
    let mut tails: [[usize; 2]; 9] = [[0, 0]; 9]; // Stores the coordinates of the tail
    for step in steps {
        let parts: Vec<&str> = step.split(" ").collect();
        let direction: char = parts[0].parse().expect("Not a character");
        let amount: usize = parts[1].parse().expect("Not a number");
        // Repeats this movement the amount the movement should be reported
        for _ in 0..amount {
            // Moves the head based on the direction given
            if direction == 'U' {
                head[1] += 1;
            } else if direction == 'D' {
                if head[1] == 0 {
                    for x in 0..positions.len() {
                        let mut result = vec![[0, 0]];
                        for item in &positions[x] {
                            result.push(item.clone());
                        }
                        positions[x] = result;
                    }
                    head[1] = 1;
                    for x in 0..tails.len() {
                        tails[x][1] += 1;
                    }
                }
                head[1] -= 1;
            } else if direction == 'R' {
                head[0] += 1;
            } else {
                if head[0] == 0 {
                    let mut result = vec![[0, 0]];
                    for _ in 0..head[1] {
                        result.push([0, 0]);
                    }
                    let mut new_positions = vec![result];
                    for x in positions {
                        new_positions.push(x);
                    }
                    positions = new_positions;
                    head[0] = 1;
                    for x in 0..tails.len() {
                        tails[x][0] += 1;
                    }
                }
                head[0] -= 1;
            }
            // Follows the tail
            let mut head_data = head;
            for x in 0..tails.len() {
                let mut tail = tails[x];
                let distx = (head_data[0] as i32) - (tail[0] as i32);
                let disty = (head_data[1] as i32) - (tail[1] as i32);
                if distx.abs() > 1 {
                    tail[0] = ((tail[0] as i32) + distx.signum()) as usize;
                    if disty != 0 {
                        tail[1] = ((tail[1] as i32) + disty.signum()) as usize;
                    }
                } else if disty.abs() > 1 {
                    tail[1] = ((tail[1] as i32) + disty.signum()) as usize;
                    if distx != 0 {
                        tail[0] = ((tail[0] as i32) + distx.signum()) as usize;
                    }
                }
                tails[x] = tail;
                head_data = tail;
            }
            // Adds the first tail to the coordinate list
            let part1_tail = tails[0];
            while positions.len() <= part1_tail[0] {
                positions.push(vec![]);
            }
            while positions[part1_tail[0]].len() <= part1_tail[1] {
                positions[part1_tail[0]].push([0, 0]);
            }
            positions[part1_tail[0]][part1_tail[1]][0] += 1;
            // Checks if this was a unique spot
            if positions[part1_tail[0]][part1_tail[1]][0] == 1 {
                part1 += 1;
            }
            // Adds the last tail to the coordinate list
            let part2_tail = tails[8];
            while positions.len() <= part2_tail[0] {
                positions.push(vec![]);
            }
            while positions[part2_tail[0]].len() <= part2_tail[1] {
                positions[part2_tail[0]].push([0, 0]);
            }
            positions[part2_tail[0]][part2_tail[1]][1] += 1;
            // Checks if this was a unique spot
            if positions[part2_tail[0]][part2_tail[1]][1] == 1 {
                part2 += 1;
            }
        }
    }
    println!("Answer for day 9 part 1 is {}.", part1);
    assert_eq!(part1, 6044);
    println!("Answer for day 9 part 2 is {}.", part2);
    assert_eq!(part2, 2384);
}