use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val),
            Self::List(list) => {
                write!(f, "[")?;
                if !list.is_empty() {
                    for val in &list[..list.len() - 1] {
                        val.fmt(f)?;
                        write!(f, ",")?;
                    }
                    list[list.len() - 1].fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
}

fn main() {
    let packets = parse_input();

    let sum_of_correct_indices =
        packets
            .chunks(2)
            .enumerate()
            .fold(0 as usize, |accum, (index, values)| {
                if is_correct(&values[0], &values[1]) {
                    accum + index + 1
                } else {
                    accum
                }
            });

    println!("Sum of indices: {}", sum_of_correct_indices);
}

fn is_correct(left: &Value, right: &Value) -> bool {
    match is_correct_helper(left, right) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => panic!("Equal packets received left: {:?} right: {:?}", left, right),
    }
}

fn is_correct_helper(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => left.cmp(right),
        (Value::List(_), Value::Int(right)) => {
            is_correct_helper(&left, &Value::List(vec![Value::Int(*right)]))
        }
        (Value::Int(left), Value::List(_)) => {
            is_correct_helper(&Value::List(vec![Value::Int(*left)]), right)
        }
        (Value::List(left), Value::List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                match is_correct_helper(left, right) {
                    Ordering::Equal => (),
                    less_or_greater => return less_or_greater,
                };
            }
            left.len().cmp(&right.len())
        }
    }
}

fn parse_input() -> Vec<Value> {
    aoc::read_lines("input.txt")
        .expect("Couldn't open input.txt for reading")
        .map_while(|line| line.ok())
        .filter(|text| !text.is_empty())
        .map(|text| parse_line(&text))
        .collect()
}

fn parse_line(text: &str) -> Value {
    assert!(text.starts_with('['));
    parse_list(text).0
}

fn parse_list(text: &str) -> (Value, &str) {
    let mut list = Vec::new();

    let mut remaining = &text[1..];

    loop {
        match remaining.chars().next().unwrap() {
            '[' => {
                let (sublist, new_remaining) = parse_list(&remaining);
                remaining = new_remaining;
                list.push(sublist);
            }
            ']' => return (Value::List(list), &remaining[1..]),
            ',' => remaining = &remaining[1..],
            _ => {
                list.push(Value::Int(
                    remaining.split_once([',', ']']).unwrap().0.parse().unwrap(),
                ));
                remaining = &remaining[remaining.find(&[',', ']']).unwrap()..];
            }
        }
    }
}
