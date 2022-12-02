use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut elves: Vec<i32> = Vec::new();
    let mut current_calories = 0;

    for line in lines {
        match line {
            Ok(text) => match text.as_str() {
                "" => {
                    elves.push(current_calories);
                    current_calories = 0;
                }
                _ => {
                    current_calories += text
                        .parse::<i32>()
                        .unwrap_or_else(|_| panic!("Encountered a non-numeric value {}", text))
                }
            },
            Err(_) => break,
        }
    }

    assert!(elves.len() >= 3);

    elves.sort();
    println!("Largest number of calories: {}", elves[elves.len() - 1]);
    println!(
        "Sum of top three: {}",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    )
}
