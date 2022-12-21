use std::collections::HashMap;
struct Folder<'a> {
    folders: HashMap<&'a str, Folder<'a>>,
    files: HashMap<&'a str, i64>,
    size: i64,
}
impl<'a> Folder<'a> {
    pub fn new() -> Self {
        Self {
            folders: HashMap::new(),
            size: 0,
            files: HashMap::new(),
        }
    }
    // If size is -1 then this is a folder otherwise it makes it a file
    pub fn new_file(&mut self, location: &Vec<&'a str>, size: i64, name: &'a str) -> i64 {
        if location.len() == 0 {
            if size == -1 {
                self.folders.insert(name, Folder::new());
                return 0;
            } else {
                let previous_size = match self.files.get(name) {
                    None => 0,
                    Some(x) => *x,
                };
                self.files.insert(name, size);
                self.size += size - previous_size;
                return size - previous_size;
            }
        }
        let folder = location[0];
        if !self.folders.contains_key(folder) {
            self.folders.insert(folder, Folder::new());
        }
        let folder = self.folders.get_mut(folder).expect("Unknown Error");
        let return_size: i64;
        if location.len() == 1 {
            return_size = folder.new_file(&vec![], size, name);
        } else {
            return_size = folder.new_file(&location[1..location.len()].to_vec(), size, name);
        }
        self.size += return_size;
        return return_size;
    }
    // Calculates the total of the size of folders under a certain size
    pub fn part1(&self, max_size: i64) -> i64 {
        let mut total = 0;
        if self.size <= max_size {
            total += self.size;
        }
        for (_, value) in self.folders.iter() {
            total += value.part1(max_size);
        }
        return total;
    }
    // Calculates the smallest folder that is at least the size of smallest_file
    pub fn part2(&self, smallest_file: i64) -> i64 {
        let mut smallest = i64::MAX;
        for (_, value) in self.folders.iter() {
            let value = value.part2(smallest_file);
            if value < smallest {
                smallest = value;
            }
        }
        if self.size < smallest && self.size >= smallest_file {
            smallest = self.size;
        }
        return smallest;
    }
}
// Used to get the data
const DATA: &str = include_str!("../data/day7.txt");
pub fn main() {
    let data: Vec<&str> = DATA.split("\n").collect();
    let mut folder_data: Folder = Folder::new(); // Stores the folder data
    let mut directory: Vec<&str> = vec![]; // Stores the current directory
    let mut counter = 0;
    // Calculates the folder structure and the size of every folder
    while counter < data.len() {
        let line = data[counter];
        // Changes the directory when the start is cd
        if line.starts_with("$ cd") {
            let directory_move = &line[5..];
            if directory_move == ".." {
                directory.pop();
            } else if directory_move == "/" {
                directory.clear();
            } else {
                directory.push(directory_move);
            }
        } else if line.starts_with("$ ls") {
            // Takes the result of the ls commands and adds the data to the folder structure
            while counter + 1 < data.len() {
                let line = data[counter + 1];
                if line.starts_with("$") {
                    break;
                }
                let line: Vec<&str> = line.split(" ").collect();
                if line[0] == "dir" {
                    folder_data.new_file(&directory, -1, line[1]);
                } else {
                    folder_data.new_file(&directory, line[0].parse().expect("NAN"), line[1]);
                }
                counter += 1;
            }
        }
        counter += 1;
    }
    let part1 = folder_data.part1(100000);
    println!("Answer for day 7 part 1 is {part1}.");
    assert_eq!(part1, 1886043);
    let part2 = folder_data.part2(folder_data.size - 40000000);
    println!("Answer for day 7 part 2 is {}.", part2);
    assert_eq!(part2, 3842121);
}