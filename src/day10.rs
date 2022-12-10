// Used to get the data
const DATA: &str = include_str!("../data/day10.txt");
struct Clock {
    cycle: i32,
    register: i32,
    part1: i32,
    part2: String,
}
impl Clock {
    pub fn new() -> Self {
        return Self {
            cycle: 0,
            register: 1,
            part1: 0,
            part2: String::new(),
        };
    }
    // Used to simlate a clock cycle
    pub fn tick(&mut self) {
        self.cycle += 1;
        // Adds to part1 the total if needed
        if (self.cycle - 20) % 40 == 0 {
            self.part1 += self.cycle * self.register;
        }
        // Adds an entry to the display
        if ((self.cycle % 40) - self.register - 1).abs() <= 1 {
            self.part2.push('#');
        } else {
            self.part2.push('.');
        }
        if self.cycle % 40 == 0 {
            self.part2.push('\n');
        }
    }
    pub fn addx(&mut self, num: i32) {
        self.tick();
        self.tick();
        self.register += num;
    }
}
pub fn main() {
    let steps = DATA.split("\n");
    let mut clock = Clock::new();
    // Goes through every instruction
    for step in steps {
        if step.contains("noop") {
            clock.tick();
        } else if step.contains("addx") {
            let num: i32 = step
                .split(" ")
                .nth(1)
                .expect("Missing number in addx")
                .parse()
                .expect("addx number is invalid");
            clock.addx(num);
        }
    }
    println!("Answer for day 10 part 1 is {}.", clock.part1);
    assert_eq!(clock.part1, 12640);
    print!("Answer for day 10 part 2 is:\n{}", clock.part2);
    assert_eq!(clock.part2, "####.#..#.###..####.#....###....##.###.#\n#....#..#.#..#....#.#....#..#....#.#..##\n###..####.###....#..#....#..#....#.#..##\n#....#..#.#..#..#...#....###.....#.###..\n#....#..#.#..#.#....#....#.#..#..#.#.#.#\n####.#..#.###..####.####.#..#..##..#..#.\n");
}
