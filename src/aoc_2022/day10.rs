use crate::{prelude::*, std_iter};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        |i| preceded(tag("addx "), parse_i64)(i).and_then(|(i, v)| Ok((i, Instruction::Addx(v)))),
        |i| tag("noop")(i).and_then(|(i, _)| Ok((i, Instruction::Noop))),
    ))(i)
}

struct MachineState {
    x: i64,
    pipeline: VecDeque<(usize, Instruction)>,
}

impl MachineState {
    fn new() -> Self {
        MachineState {
            x: 1,
            pipeline: VecDeque::new(),
        }
    }

    fn cycle(&mut self, instructions: &mut impl Iterator<Item = Instruction>) -> bool {
        // Start loading
        if self.pipeline.is_empty() {
            if let Some(instruction) = instructions.next() {
                self.load_instruction(instruction);
            }
        }
        if self.pipeline.is_empty() {
            return false;
        }
        let (to_execute, new_pipeline) = self.process_pipeline();
        self.pipeline = new_pipeline;
        for instruction in to_execute {
            self.process(&instruction);
        }
        true
    }

    fn load_instruction(&mut self, instruction: Instruction) {
        let cycle = match instruction {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        };
        self.pipeline.push_back((cycle, instruction));
    }

    fn process_pipeline(&self) -> (Vec<Instruction>, VecDeque<(usize, Instruction)>) {
        let mut to_execute = vec![];
        let new_pipeline = self
            .pipeline
            .iter()
            .filter_map(|(cycle, instruction)| {
                if *cycle == 1 {
                    to_execute.push(*instruction);
                    None
                } else {
                    Some((*cycle - 1, *instruction))
                }
            })
            .collect();
        (to_execute, new_pipeline)
    }

    fn process(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Addx(v) => self.x += v,
            Instruction::Noop => (),
        }
    }
}

pub fn part1() {
    let mut instructions = std_iter!(Lines).map(|l| parse_instruction(&l).unwrap().1);
    let mut machine = MachineState::new();
    let mut cycle = 0;
    let mut sum = 0;
    loop {
        cycle += 1;
        eprintln!("cycle {}, x={}", cycle, machine.x);
        if (cycle - 20) % 40 == 0 {
            sum += cycle * machine.x;
        }
        if !machine.cycle(&mut instructions) {
            break;
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let mut instructions = std_iter!(Lines).map(|l| parse_instruction(&l).unwrap().1);
    let mut machine = MachineState::new();
    let mut cycle = 0;
    let mut screen = vec![vec!['.'; 40]; 6];

    loop {
        let col = cycle % 40;
        let row = cycle / 40;
        if machine.x - 1 <= col && col <= machine.x + 1 {
            screen[row as usize][col as usize] = '#';
        }

        cycle += 1;

        if !machine.cycle(&mut instructions) {
            break;
        }
    }
    for row in screen {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}
