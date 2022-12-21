// Used to get the data
const DATA: &str = include_str!("../data/day11.txt");
enum Operation {
    Add,
    Multiply,
    Power,
}
struct Monkey {
    items: Vec<i64>,
    operation: Operation, // Stores the operation the monkey does on the items worry level
    operation_amount: i64,
    test: i64, // Stores the divisibility test
    test_true: usize, // The monkey to throw to when the test comes back true
    test_false: usize, // The monkey to throw to when the test comes back false
    inspections: usize,
}
struct ThrowItem {
    worry: i64,
    monkey: usize,
}
impl Monkey {
    // Parses the string and returns a monkey object for that string
    pub fn new(descriptor: &str) -> Self {
        let mut description = descriptor.split("\n");
        // Finds all the items for the monkey
        let items: Vec<i64> = (&description.nth(1).expect("Bad Item list")[18..])
            .split(", ")
            .map(|x| x.parse::<i64>().expect("Invalid item list"))
            .collect();
        let mut operation = Operation::Power;
        let mut operation_amount = 2;
        let operation_str = &description.nth(0).expect("Bad Operation syntax")[23..];
        // Figures out what the operation is
        if !operation_str.contains("old") {
            let mut chars = operation_str.chars();
            operation = match chars.nth(0).expect("Bad Operation") {
                '+' => Operation::Add,
                '*' => Operation::Multiply,
                _ => Operation::Power,
            };
            chars.next();
            operation_amount = chars.as_str().parse().expect("Bad operation number");
        }
        // Finds out what the divisor is
        let test = (&description.nth(0).expect("Bad Test")[21..])
            .parse::<i64>()
            .expect("Bad Divisor");
        let test_true = (&description.nth(0).expect("Bad Test")[29..])
            .parse::<usize>()
            .expect("Bad Divisor");
        let test_false = (&description.nth(0).expect("Bad Test")[30..])
            .parse::<usize>()
            .expect("Bad Divisor");
        Self {
            items,
            operation,
            operation_amount,
            test,
            test_true,
            test_false,
            inspections: 0,
        }
    }
    // Inspects an object in the monkeys hand and returns its worry and target
    pub fn inspect(&mut self, part1: bool) -> Option<ThrowItem> {
        if self.items.len() == 0 {
            return None;
        }
        let mut item = self.items.pop().expect("Invalid inspection");
        self.inspections += 1;
        match self.operation {
            Operation::Add => {
                item += self.operation_amount;
            }
            Operation::Multiply => {
                item *= self.operation_amount;
            }
            Operation::Power => {
                item *= item;
            }
        }
        if part1 {
            item /= 3;
        }
        return Some(ThrowItem {
            worry: item,
            monkey: if item % self.test == 0 {
                self.test_true
            } else {
                self.test_false
            },
        });
    }
    // Used to give a monkey an object
    pub fn give(&mut self, worry: i64) {
        self.items.push(worry);
    }
}
pub fn main() {
    let steps = DATA.split("\n\n");
    // Parses all the monkeys
    let mut monkeys: Vec<Monkey> = vec![];
    for new_monkey in steps {
        monkeys.push(Monkey::new(new_monkey));
    }
    // Goes through the 20 rounds
    for _ in 0..20 {
        for count in 0..monkeys.len() {
            loop {
                let object = monkeys[count].inspect(true);
                if object.is_none() {
                    break;
                }
                let object = object.expect("Bad loop");
                monkeys[object.monkey].give(object.worry);
            }
        }
    }
    // Finds the 2 bussiest monkeys for part1
    let mut part1: Vec<usize> = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect();
    part1.sort();
    part1.reverse();
    let part1 = part1[0] * part1[1];
    println!("Answer for day 11 part 1 is {}.", part1);
    assert_eq!(part1, 90882);
    let steps = DATA.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];
    for new_monkey in steps {
        let monkey = Monkey::new(new_monkey);
        monkeys.push(monkey);
    }
    // Finds the lowest common multiple of all the monkeys divisor tests
    let lcm: i64 = monkeys
        .iter()
        .map(|x| x.test)
        .product();
    // Simulates a thousand round
    for _ in 0..10000 {
        for count in 0..monkeys.len() {
            loop {
                let object = monkeys[count].inspect(false);
                if object.is_none() {
                    break;
                }
                let object = object.expect("Bad loop");
                // This makes the number much smaller by making the max be the least common multiple of all the factors
                monkeys[object.monkey].give(object.worry % lcm);
            }
        }
    }
    // Finds the 2 bussiest monkeys for part2
    let mut part2: Vec<usize> = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect();
    part2.sort();
    part2.reverse();
    let part2 = part2[0] * part2[1];
    println!("Answer for day 11 part 2 is:{}.", part2);
    assert_eq!(part2, 30893109657);
}