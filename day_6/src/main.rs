use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input_buf = std::fs::read_to_string("input.txt").expect("Couldn't open input.txt for reading");

    println!("{}", get_startcode_index(&input_buf));
}

fn get_startcode_index(input_buf: &str) -> usize {
    for (index, (first, second, third, fourth)) in input_buf.chars().tuple_windows().enumerate() {
        let mut set = HashSet::new();
        set.insert(first);
        set.insert(second);
        set.insert(third);
        set.insert(fourth);

        if set.len() == 4 {
            return index + 4;
        }
    }

    panic!("Start code not found!");
}
