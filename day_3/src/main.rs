fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut total_priority: u32 = 0;

    for line in lines {
        match line {
            Ok(text) => {
                let (first, second) = text.split_at(text.len() / 2);

                for char in first.chars() {
                    if let Some(_) = second.find(char) {
                        total_priority += get_priority(char);
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }

    println!("Total priority: {}", total_priority);
}

fn get_priority(item: char) -> u32 {
    let ascii_val = item as u32;
    match ascii_val {
        65..=90 => { ascii_val - 38 },
        97..=122 => { ascii_val - 96 },
        _ => panic!("Unknown item of type {} encountered", item)
    }
}
