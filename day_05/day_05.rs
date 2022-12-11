const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Clone, Copy)]
struct Crate {
    id: char,
}

type Stacks = Vec<Vec<Crate>>;
fn parse_stacks(input: &str) -> Stacks {
    let lines = input.lines();
    let as_crates: Stacks = lines
        .take_while(|line| !line.trim().starts_with(char::is_numeric))
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chars| Crate { id: chars[1] })
                .collect()
        })
        .collect();

    let num_stacks = &as_crates.first().unwrap().len();
    as_crates.iter().rev().fold(
        (0..*num_stacks).map(|_| vec![]).collect(),
        |mut result, crates| {
            for (i, elem) in crates.iter().enumerate() {
                if elem.id != ' ' {
                    result[i].push(*elem)
                }
            }
            result
        },
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Move { count: u32, from: usize, to: usize },
    MoveBatch { count: u32, from: usize, to: usize },
}

#[derive(Debug)]
enum InstructionParseMode {
    CrateMover9000,
    CrateMover9001,
}

type Instructions = Vec<Instruction>;
fn parse_instructions(input: &str, mode: &InstructionParseMode) -> Instructions {
    let instructions_string = input
        .split("\n")
        .skip_while(|ch| !ch.is_empty())
        .collect::<Vec<&str>>()
        .join("\n");
    instructions_string
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            parts.next();
            let count = parts.next().unwrap().parse().unwrap();
            parts.next();
            let from = parts.next().unwrap().parse().unwrap();
            parts.next();
            let to = parts.next().unwrap().parse().unwrap();
            match mode {
                InstructionParseMode::CrateMover9000 => Instruction::Move { count, from, to },
                InstructionParseMode::CrateMover9001 => Instruction::MoveBatch { count, from, to },
            }
        })
        .collect()
}

fn apply_instructions(stacks: &mut Stacks, instructions: Instructions) {
    for instruction in instructions {
        match instruction {
            Instruction::Move { count, from, to } => {
                for _ in 0..count {
                    let source_crate = stacks[from - 1].pop().unwrap();
                    stacks[to - 1].push(source_crate);
                }
            }
            Instruction::MoveBatch { count, from, to } => {
                let batch: Vec<Crate> = (0..count)
                    .map(|_| stacks[from - 1].pop().unwrap())
                    .collect();
                for elem in batch.iter().rev() {
                    stacks[to - 1].push(*elem);
                }
            }
        }
    }
}

fn run_simulation(input: &str, mode: InstructionParseMode) {
    let mut stacks = parse_stacks(input);
    let instructions = parse_instructions(input, &mode);

    apply_instructions(&mut stacks, instructions);
    let result: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap().id)
        .collect();
    println!("Result with {mode:?}: {result}")
}

fn main() {
    run_simulation(INPUT, InstructionParseMode::CrateMover9000);
    run_simulation(INPUT, InstructionParseMode::CrateMover9001);
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    ";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_stacks(SAMPLE.trim_matches('\n')),
            vec![
                vec![Crate { id: 'Z' }, Crate { id: 'N' }],
                vec![Crate { id: 'M' }, Crate { id: 'C' }, Crate { id: 'D' }],
                vec![Crate { id: 'P' }],
            ]
        );
    }

    #[test]
    fn test_parse_instructions_9000() {
        assert_eq!(
            parse_instructions(
                SAMPLE.trim_matches('\n'),
                &InstructionParseMode::CrateMover9000
            ),
            vec![
                Instruction::Move {
                    count: 1,
                    from: 2,
                    to: 1
                },
                Instruction::Move {
                    count: 3,
                    from: 1,
                    to: 3
                },
                Instruction::Move {
                    count: 2,
                    from: 2,
                    to: 1
                },
                Instruction::Move {
                    count: 1,
                    from: 1,
                    to: 2
                },
            ]
        )
    }

    #[test]
    fn test_apply() {
        let mut stacks = parse_stacks(SAMPLE.trim_matches('\n'));
        let instructions = parse_instructions(
            SAMPLE.trim_matches('\n'),
            &InstructionParseMode::CrateMover9000,
        );
        apply_instructions(&mut stacks, instructions);
        assert_eq!(
            stacks,
            vec![
                vec![Crate { id: 'C' }],
                vec![Crate { id: 'M' }],
                vec![
                    Crate { id: 'P' },
                    Crate { id: 'D' },
                    Crate { id: 'N' },
                    Crate { id: 'Z' }
                ],
            ]
        );
    }

    #[test]
    fn test_apply_batch() {
        let mut stacks = parse_stacks(SAMPLE.trim_matches('\n'));
        let instructions = parse_instructions(
            SAMPLE.trim_matches('\n'),
            &InstructionParseMode::CrateMover9001,
        );
        apply_instructions(&mut stacks, instructions);
        assert_eq!(
            stacks,
            vec![
                vec![Crate { id: 'M' }],
                vec![Crate { id: 'C' }],
                vec![
                    Crate { id: 'P' },
                    Crate { id: 'Z' },
                    Crate { id: 'N' },
                    Crate { id: 'D' }
                ],
            ]
        );
    }
}
