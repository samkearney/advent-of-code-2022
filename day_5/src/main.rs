use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Lines};

fn main() {
    let mut lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut pads = get_initial_crates(&mut lines);

    lines.next(); // Skip the empty line

    for line in lines {
        match line {
            Ok(text) => execute_move_order(&text, &mut pads),
            Err(_) => break
        }
    }

    let top_crates: String = pads.iter().map(|q| q.front().unwrap()).collect();
    println!("{}", top_crates);
}

fn get_initial_crates(lines: &mut Lines<BufReader<File>>) -> Vec<VecDeque<char>> {
    let mut result: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for line in &mut lines
        .map(|line| line.unwrap())
        .take_while(|line| !line.starts_with(" 1   2   3   4   5   6   7   8   9"))
    {
        for (index, val) in line.chars().skip(1).step_by(4).enumerate() {
            if val.is_ascii_uppercase() {
                result[index].push_back(val);
            }
        }
    }

    result
}

fn execute_move_order(order: &str, pads: &mut [VecDeque<char>]) {
    let mut tokens = order.split(' ');

    tokens.next();
    let count: i32 = tokens.next().unwrap().parse().unwrap();

    tokens.next();
    let src: usize = tokens.next().unwrap().parse().unwrap();

    tokens.next();
    let dest: usize = tokens.next().unwrap().parse().unwrap();

    for _ in 0..count {
        let crate_id = pads[src - 1].pop_front().unwrap();
        pads[dest - 1].push_front(crate_id);
    }
}
