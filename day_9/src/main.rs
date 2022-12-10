use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn move_left(&mut self) {
        self.0 -= 1;
    }

    fn move_right(&mut self) {
        self.0 += 1;
    }

    fn move_up(&mut self) {
        self.1 += 1;
    }

    fn move_down(&mut self) {
        self.1 -= 1;
    }
}

fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut rope = vec![Position(0, 0); 10];

    for line in lines {
        match line {
            Ok(text) => {
                let mut tokens = text.split(' ');
                match tokens.next().unwrap() {
                    "L" => {
                        for _ in 0..tokens.next().unwrap().parse().unwrap() {
                            rope[0].move_left();
                            update_rope(&mut rope);
                            tail_positions.insert(*rope.last().unwrap());
                        }
                    }
                    "R" => {
                        for _ in 0..tokens.next().unwrap().parse().unwrap() {
                            rope[0].move_right();
                            update_rope(&mut rope);
                            tail_positions.insert(*rope.last().unwrap());
                        }
                    }
                    "U" => {
                        for _ in 0..tokens.next().unwrap().parse().unwrap() {
                            rope[0].move_up();
                            update_rope(&mut rope);
                            tail_positions.insert(*rope.last().unwrap());
                        }
                    }
                    "D" => {
                        for _ in 0..tokens.next().unwrap().parse().unwrap() {
                            rope[0].move_down();
                            update_rope(&mut rope);
                            tail_positions.insert(*rope.last().unwrap());
                        }
                    }
                    _ => panic!("Unknown move command received"),
                }
            }
            Err(_) => break,
        }
    }

    println!("Number of tail positions: {}", tail_positions.len());
}

fn update_rope(rope: &mut [Position]) {
    for link_index in 1..rope.len() {
        rope[link_index] = get_new_tail_position(&rope[link_index - 1], &rope[link_index]);
    }
}

fn get_new_tail_position(new_head_pos: &Position, old_tail_pos: &Position) -> Position {
    let horiz_distance = (new_head_pos.0 - old_tail_pos.0).abs();
    let vert_distance = (new_head_pos.1 - old_tail_pos.1).abs();

    if horiz_distance <= 1 && vert_distance <= 1 {
        // No move needed
        return *old_tail_pos;
    }

    let (mut new_tail_horiz, mut new_tail_vert) = (old_tail_pos.0, old_tail_pos.1);
    match new_head_pos.0.cmp(&old_tail_pos.0) {
        Ordering::Less => new_tail_horiz -= 1,
        Ordering::Greater => new_tail_horiz += 1,
        _ => {}
    };

    match new_head_pos.1.cmp(&old_tail_pos.1) {
        Ordering::Less => new_tail_vert -= 1,
        Ordering::Greater => new_tail_vert += 1,
        _ => {}
    };

    Position(new_tail_horiz, new_tail_vert)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_moves_work() {
        assert!(get_new_tail_position(&Position(2, 0), &Position(0, 0)) == Position(1, 0));
        assert!(get_new_tail_position(&Position(-2, 0), &Position(0, 0)) == Position(-1, 0));
        assert!(get_new_tail_position(&Position(0, 2), &Position(0, 0)) == Position(0, 1));
        assert!(get_new_tail_position(&Position(0, -2), &Position(0, 0)) == Position(0, -1));

        assert!(get_new_tail_position(&Position(4, 4), &Position(2, 4)) == Position(3, 4));
        assert!(get_new_tail_position(&Position(-4, -4), &Position(-4, -2)) == Position(-4, -3));
    }

    #[test]
    fn diagonal_non_moves_work() {
        assert!(get_new_tail_position(&Position(1, 1), &Position(0, 0)) == Position(0, 0));
        assert!(get_new_tail_position(&Position(0, 0), &Position(1, 1)) == Position(1, 1));
        assert!(get_new_tail_position(&Position(0, 1), &Position(1, 0)) == Position(1, 0));
        assert!(get_new_tail_position(&Position(1, 0), &Position(0, 1)) == Position(0, 1));
    }

    #[test]
    fn diagonal_moves_work() {
        assert!(get_new_tail_position(&Position(1, 2), &Position(0, 0)) == Position(1, 1));
        assert!(get_new_tail_position(&Position(-1, 2), &Position(0, 0)) == Position(-1, 1));
        assert!(get_new_tail_position(&Position(1, -2), &Position(0, 0)) == Position(1, -1));
        assert!(get_new_tail_position(&Position(-1, -2), &Position(0, 0)) == Position(-1, -1));

        assert!(get_new_tail_position(&Position(2, 1), &Position(0, 0)) == Position(1, 1));
        assert!(get_new_tail_position(&Position(-2, 1), &Position(0, 0)) == Position(-1, 1));
        assert!(get_new_tail_position(&Position(2, -1), &Position(0, 0)) == Position(1, -1));
        assert!(get_new_tail_position(&Position(-2, -1), &Position(0, 0)) == Position(-1, -1));
    }
}
