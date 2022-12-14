struct ThrowOperation {
    item: i32,
    target: usize,
}

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    test_divisor: i32,
    true_target: usize,
    false_target: usize,
    times_inspected: u32,
}

impl Monkey {
    pub fn new(
        items: Vec<i32>,
        op: impl Fn(i32) -> i32 + 'static,
        test_divisor: i32,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Monkey {
            items,
            op: Box::new(op),
            test_divisor,
            true_target,
            false_target,
            times_inspected: 0,
        }
    }

    pub fn times_inspected(&self) -> u32 {
        self.times_inspected
    }

    pub fn process_round(&mut self) -> Vec<ThrowOperation> {
        self.times_inspected += self.items.len() as u32;

        self.items
            .drain(..)
            .map(|item| {
                // Inspect: Increase worry level
                let item = (self.op)(item);

                // Get bored: Divide worry level
                let item = item / 3;

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

    pub fn catch_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

fn main() {
    let mut monkeys = [
        Monkey::new(vec![75, 75, 98, 97, 79, 97, 64], |old| old * 13, 19, 2, 7),
        Monkey::new(vec![50, 99, 80, 84, 65, 95], |old| old + 2, 3, 4, 5),
        Monkey::new(
            vec![96, 74, 68, 96, 56, 71, 75, 53],
            |old| old + 1,
            11,
            7,
            3,
        ),
        Monkey::new(vec![83, 96, 86, 58, 92], |old| old + 8, 17, 6, 1),
        Monkey::new(vec![99], |old| old * old, 5, 0, 5),
        Monkey::new(vec![60, 54, 83], |old| old + 4, 2, 2, 0),
        Monkey::new(vec![77, 67], |old| old * 17, 13, 4, 1),
        Monkey::new(vec![95, 65, 58, 76], |old| old + 5, 7, 3, 6),
    ];

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let throws = monkeys[monkey_idx].process_round();
            for throw in throws {
                monkeys[throw.target].catch_item(throw.item);
            }
        }
    }

    let mut inspection_totals: Vec<u32> = monkeys
        .iter()
        .map(|monkey| monkey.times_inspected())
        .collect();
    inspection_totals.sort();

    println!("{:?}", inspection_totals);
    println!(
        "Monkey business: {}",
        inspection_totals[inspection_totals.len() - 1]
            * inspection_totals[inspection_totals.len() - 2]
    );
}
