use itertools::Itertools;

fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut total_priority_misplaced_items: u32 = 0;
    let mut total_priority_badges: u32 = 0;

    for line_group in &lines.chunks(3) {
        let lines: Vec<String> = line_group
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();

        if lines.len() < 3 {
            break;
        }

        // Get misplaced items for each sack
        for line in &lines {
            let (first, second) = line.split_at(line.len() / 2);

            for item in first.chars() {
                if let Some(_) = second.find(item) {
                    total_priority_misplaced_items += get_priority(item);
                    break;
                }
            }
        }

        // Get the badge for the group of three elves
        for item in lines[0].chars() {
            if let (Some(_), Some(_)) = (lines[1].find(item), lines[2].find(item)) {
                total_priority_badges += get_priority(item);
                break;
            }
        }
    }

    println!("Total priority of misplaced items: {}", total_priority_misplaced_items);
    println!("Total priority of badges: {}", total_priority_badges);
}

fn get_priority(item: char) -> u32 {
    let ascii_val = item as u32;
    match ascii_val {
        65..=90 => ascii_val - 38,
        97..=122 => ascii_val - 96,
        _ => panic!("Unknown item of type {} encountered", item),
    }
}
