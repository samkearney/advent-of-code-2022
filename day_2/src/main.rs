use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut total_score = 0;

    for line in lines {
        match line {
            Ok(text) => {
                let mut moves = text.split(' ').map(|slice| slice.chars().next().unwrap());
                total_score += get_score(moves.next().unwrap(), moves.next().unwrap());
            }
            Err(_) => break,
        }
    }

    println!("Total score: {}", total_score);
}

fn get_score(opponent_move: char, your_strategy: char) -> i32 {
    let opponent_move = match opponent_move {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!(
            "Received unknown value {} as opponent's move",
            opponent_move
        ),
    };

    let (your_move, win_score) = match your_strategy {
        'X' => (get_loss(opponent_move), 0),
        'Y' => (get_draw(opponent_move), 3),
        'Z' => (get_win(opponent_move), 6),
        _ => panic!("Received unknown value {} as your strategy", your_strategy),
    };

    your_move + win_score
}

fn get_win(opponent_move: i32) -> i32 {
    if opponent_move == 3 {
        1
    } else {
        opponent_move + 1
    }
}

fn get_draw(opponent_move: i32) -> i32 {
    opponent_move
}

fn get_loss(opponent_move: i32) -> i32 {
    if opponent_move == 1 {
        3
    } else {
        opponent_move - 1
    }
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
