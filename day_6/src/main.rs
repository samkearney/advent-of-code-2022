use std::collections::HashSet;

fn main() {
    let input_buf = std::fs::read_to_string("input.txt").expect("Couldn't open input.txt for reading");

    println!("Start-of-packet: {}", get_marker(&input_buf, 4));
    println!("Start-of-message: {}", get_marker(&input_buf, 14));
}

fn get_marker(input_buf: &str, num_unique: usize) -> usize {
    let mut iter = input_buf.chars().enumerate();

    while let Some((index, char)) = iter.next() {
        let mut set = HashSet::new();
        set.insert(char);
        for (_, char) in iter.clone().take(num_unique - 1) {
            set.insert(char);
        }

        if set.len() == num_unique {
            return index + num_unique;
        }
    }

    panic!("Marker not found!");
}
