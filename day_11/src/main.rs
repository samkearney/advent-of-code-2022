use std::fs::File;
use std::io;

type Item = i64;

struct ThrowOperation {
    item: Item,
    target: usize,
}

enum WorryIncreaseOperation {
    Add(Item),
    Mult(Item),
    Square,
}

struct Monkey {
    items: Vec<Item>,
    op: WorryIncreaseOperation,
    test_divisor: Item,
    true_target: usize,
    false_target: usize,
    times_inspected: u64,
}

impl Monkey {
    pub fn new(
        items: Vec<Item>,
        op: WorryIncreaseOperation,
        test_divisor: Item,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Monkey {
            items,
            op,
            test_divisor,
            true_target,
            false_target,
            times_inspected: 0,
        }
    }

    pub fn times_inspected(&self) -> u64 {
        self.times_inspected
    }

    pub fn test_divisor(&self) -> Item {
        self.test_divisor
    }

    pub fn process_round(&mut self, common_mult: Option<i64>) -> Vec<ThrowOperation> {
        self.times_inspected += self.items.len() as u64;

        self.items
            .drain(..)
            .map(|item| {
                // Inspect: Increase worry level
                let item = match self.op {
                    WorryIncreaseOperation::Add(addend) => item + addend,
                    WorryIncreaseOperation::Mult(multiplier) => item * multiplier,
                    WorryIncreaseOperation::Square => item * item,
                };

                // Get bored: Divide worry level
                let item = if let Some(common_mult) = common_mult {
                    item % common_mult
                } else {
                    item / 3
                };

                // Choose target
                let target = if item % self.test_divisor == 0 {
                    self.true_target
                } else {
                    self.false_target
                };

                ThrowOperation { item, target }
            })
            .collect()
    }

    pub fn catch_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

fn main() {
    let mut monkeys = parse_input();

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let throws = monkeys[monkey_idx].process_round(None);
            for throw in throws {
                monkeys[throw.target].catch_item(throw.item);
            }
        }
    }

    let mut inspection_totals: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected())
        .collect();
    inspection_totals.sort();

    println!(
        "Monkey business for part one: {}",
        inspection_totals[inspection_totals.len() - 1]
            * inspection_totals[inspection_totals.len() - 2]
    );

    let mut monkeys = parse_input();
    let common_mult = monkeys
        .iter()
        .fold(1, |accum, monkey| accum * monkey.test_divisor());

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let throws = monkeys[monkey_idx].process_round(Some(common_mult));
            for throw in throws {
                monkeys[throw.target].catch_item(throw.item);
            }
        }
    }

    let mut inspection_totals: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected())
        .collect();
    inspection_totals.sort();

    println!(
        "Monkey business for part two: {}",
        inspection_totals[inspection_totals.len() - 1]
            * inspection_totals[inspection_totals.len() - 2]
    );
}

fn parse_input() -> Vec<Monkey> {
    let mut result = Vec::new();

    let mut input = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    while let Some(Ok(text)) = input.next() {
        if text.starts_with("Monkey") {
            result.push(get_monkey(&mut input));
        }
    }

    result
}

fn get_monkey(input: &mut io::Lines<io::BufReader<File>>) -> Monkey {
    let starting_items: Vec<Item> = get_line_required(input)
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(',')
        .map(|str| str.trim())
        .map(|str| str.parse().unwrap())
        .collect();

    let operation_line = get_line_required(input);
    let operation_str = operation_line
        .strip_prefix("  Operation: new = old ")
        .unwrap();
    let worry_operation = if operation_str == "* old" {
        WorryIncreaseOperation::Square
    } else if operation_str.starts_with('*') {
        WorryIncreaseOperation::Mult(operation_str.strip_prefix("* ").unwrap().parse().unwrap())
    } else if operation_str.starts_with('+') {
        WorryIncreaseOperation::Add(operation_str.strip_prefix("+ ").unwrap().parse().unwrap())
    } else {
        panic!("Unknown operation {}", operation_str);
    };

    let test_divisor: Item = get_line_required(input)
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    let true_target: usize = get_line_required(input)
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    let false_target: usize = get_line_required(input)
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    Monkey::new(
        starting_items,
        worry_operation,
        test_divisor,
        true_target,
        false_target,
    )
}

fn get_line_required(input: &mut io::Lines<io::BufReader<File>>) -> String {
    input
        .next()
        .expect("Malformed input: Expected line")
        .expect("Malformed input: expected line")
}
