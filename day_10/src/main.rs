use std::fs::File;
use std::io;

#[derive(Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

struct CpuState {
    current_instruction: Option<Instruction>,
    current_instruction_loaded: i32,
    reg_x: i32,
}

impl CpuState {
    pub fn new() -> Self {
        CpuState {
            current_instruction: None,
            current_instruction_loaded: 0,
            reg_x: 1,
        }
    }

    pub fn process_cycle(
        &mut self,
        cycle: i32,
        input_stream: &mut io::Lines<io::BufReader<File>>,
    ) -> bool {
        match self.current_instruction {
            None => match input_stream.next() {
                Some(result) => match result {
                    Ok(text) => {
                        self.load_instruction(cycle, &text);
                        self.process_instruction(cycle);
                    },
                    Err(_) => return false,
                },
                None => return false,
            },
            Some(_) => self.process_instruction(cycle),
        };

        true
    }

    pub fn reg_x(&self) -> i32 {
        self.reg_x
    }

    // private
    fn load_instruction(&mut self, cycle: i32, input: &str) {
        let mut tokens = input.split(' ');

        match tokens
            .next()
            .expect("Received unexpected blank line in input")
        {
            "addx" => {
                self.current_instruction = Some(Instruction::Addx(
                    tokens
                        .next()
                        .expect("addx instruction without argument")
                        .parse()
                        .expect("addx instruction with non-numeric argument"),
                ));
            }
            "noop" => {
                self.current_instruction = Some(Instruction::Noop);
            },
            unknown => panic!("Invalid instruction {} loaded from input", unknown),
        };
        self.current_instruction_loaded = cycle;
    }

    fn process_instruction(&mut self, cycle: i32) {
        match self.current_instruction {
            Some(instruction) => {
                match instruction {
                    Instruction::Addx(val) => {
                        if cycle > self.current_instruction_loaded {
                            self.reg_x += val;
                            self.current_instruction = None;
                        }
                    },
                    Instruction::Noop => {
                        self.current_instruction = None;
                    }
                }
            },
            None => panic!("No instruction to process!")
        };
    }
}

fn main() {
    let mut lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut cpu = CpuState::new();
    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    let mut signal_strength = 0;

    for cycle in 1.. {
        if interesting_cycles.contains(&cycle) {
            signal_strength += cycle * cpu.reg_x();
        }

        if !cpu.process_cycle(cycle, &mut lines) {
            break;
        }
    }

    println!("Signal strength sum: {}", signal_strength);
}
