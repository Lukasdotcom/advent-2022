// Used to get the data
const DATA: &str = include_str!("../data/day8.txt");
// Stores the data for the tree
#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
    part2: usize,
}
fn is_visible(visibility_map: &mut [usize; 10], height: usize, length: usize) -> usize {
    let visible = length + 1 - visibility_map[height as usize];
    for x in 0..height + 1 {
        visibility_map[x] = length + 1;
    }
    return visible;
}
pub fn main() {
    // Collects and formats the data
    let mut data: Vec<Vec<Tree>> = DATA.split("\n")
        .map(|x| {
            x.chars()
                .map(|y| Tree {
                    height: y.to_string().parse().expect("A non number was in input"),
                    visible: false,
                    part2: 1,
                })
                .collect()
        })
        .collect();
    let width = data[0].len();
    let height = data.len();
    // Goes through every column to calculate which trees are visible from top to bottom
    for x in 0..width {
        data[0][x].visible = true; // First row is completly visible
        let mut map: [usize; 10] = [0; 10];
        is_visible(&mut map, data[0][x].height, 0);
        for y in 1..height - 1 {
            // Goes down that column to check if it is visible
            let mut visibility = is_visible(&mut map, data[y][x].height, y);
            if visibility > y {
                data[y][x].visible = true;
                visibility -= 1;
            }
            data[y][x].part2 *= visibility;
        }
        data[height - 1][x].visible = true; // Last row is completly visible
        let mut map: [usize; 10] = [0; 10];
        is_visible(&mut map, data[height - 1][x].height, 0);
        for i in 1..height - 1 {
            // Goes up that column to check if it is visible
            let y = height - 1 - i;
            let mut visibility = is_visible(&mut map, data[y][x].height, i);
            if visibility > i {
                data[y][x].visible = true;
                visibility -= 1;
            }
            data[y][x].part2 *= visibility;
        }
    }
    let mut part1 = (width + height) * 2 - 4;
    let mut part2 = 0;
    //Goes through every row to calculate which trees are visible from left to right
    for y in 1..height - 1 {
        data[y][0].visible = true; // First column is completly visible
        let mut map: [usize; 10] = [0; 10];
        is_visible(&mut map, data[y][0].height, 0);
        for x in 1..width - 1 {
            // Goes rightwards that row to check if it is visible
            let mut visibility = is_visible(&mut map, data[y][x].height, x);
            if visibility > x {
                data[y][x].visible = true;
                visibility -= 1;
            }
            data[y][x].part2 *= visibility;
        }
        data[y][width - 1].visible = true; // Last row is completly visible
        let mut map: [usize; 10] = [0; 10];
        is_visible(&mut map, data[y][width - 1].height, 0);
        for i in 1..width - 1 {
            // Goes leftwards that column to check if it is visible
            let x = height - 1 - i;
            let mut visibility = is_visible(&mut map, data[y][x].height, i);
            if visibility > i {
                visibility -= 1;
                data[y][x].visible = true;
                part1 += 1;
            } else if data[y][x].visible {
                part1 += 1;
            }
            data[y][x].part2 *= visibility;
            // Checks if the visibility of this tree is the largest
            if part2 < data[y][x].part2 {
                part2 = data[y][x].part2;
            }
        }
    }
    println!("Answer for day 8 part 1 is {}.", part1);
    assert_eq!(part1, 1546);
    println!("Answer for day 8 part 2 is {}.", part2);
    assert_eq!(part2, 519064);
}