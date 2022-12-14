use std::str::Lines;

const INPUT: &str = include_str!("day_10.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Registers {
    x: i32,
}

#[derive(Debug)]
struct CommSimulation<'a> {
    register: Registers,
    instructions: &'a mut InstructionIterator<'a>,
    pending_instruction: Option<(u32, Instruction)>,
}

impl<'a> CommSimulation<'a> {
    fn new(instructions: &'a mut InstructionIterator<'a>) -> CommSimulation<'a> {
        CommSimulation {
            register: Registers { x: 1 },
            pending_instruction: None,
            instructions,
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(count) => self.register.x += count
        };
    }

}

impl<'a> Iterator for CommSimulation<'a> {
    type Item = Registers;

    fn next(&mut self) -> Option<Self::Item> {
        // Nothing executing currently - fetch new instruction
        if self.pending_instruction.is_none() {
            let Some(next_instruction) = self.instructions.next() else {
                return None; 
            };

            let cycle_count = match next_instruction {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            };
            self.pending_instruction = Some((cycle_count, next_instruction));
        }

        // Pending instruction
        let Some((remaining_cycles, instruction)) = &self.pending_instruction else {
            return None;
        };

        let ret = Some(self.register);

        // There are cycles remaining before it can be applied
        if *remaining_cycles > 1 {
            self.pending_instruction = Some((remaining_cycles - 1, *instruction))
        } else {
            // Last cycle, now apply
            // TODO: yield DURING, not after
            self.apply(*instruction);
            self.pending_instruction = None
        }

        ret
    }
}

trait CommSimulator<'a> {
    fn simulate(&'a mut self) -> CommSimulation<'a>;
}

impl<'a> CommSimulator<'a> for InstructionIterator<'a> {
    fn simulate(&'a mut self) -> CommSimulation<'a> {
        CommSimulation::new(self)
    }
}

#[derive(Debug)]
struct InstructionIterator<'a> {
    lines: &'a mut Lines<'a>,
}

trait Instructions<'a> {
    fn instructions(&'a mut self) -> InstructionIterator;
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Instruction;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(instruction) = self.lines.next() else { return None; };

        match instruction.trim() {
            "noop" => Some(Instruction::Noop),
            s if s.starts_with("addx") => {
                let count = s.split(' ').last().unwrap().parse().unwrap();
                Some(Instruction::Addx(count))
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> Instructions<'a> for Lines<'a> {
    fn instructions(&'a mut self) -> InstructionIterator {
        InstructionIterator { lines: self }
    }
}

fn calculate_signal_strength(input: &str) -> i32 {
    let target_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    input.lines().instructions().simulate().enumerate().filter_map(|(idx, registers)| {
        let target = idx + 1;
        if !target_cycles.contains(&target) { return None; }

        Some(target as i32 * registers.x)
    }).sum()
}

pub fn print_solution() {
    let signal_strength = calculate_signal_strength(INPUT);
    println!("Signal strength: {signal_strength}");
    let screen: Vec<char> = INPUT.lines().instructions().simulate().enumerate().map(|(idx, registers)| {
        let to_draw = [idx as i32 - 1, idx as i32, idx as i32 + 1].map(|x| x % 40);
        if to_draw.contains(&registers.x) {
            '#'
        } else {
            '.'
        }
    }).collect();
    screen.chunks(40).for_each(|line| {
        for c in line {
            print!("{}", c);
        }
        println!();
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "noop
addx 3
addx -5";

    const LARGE_SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_instructions_iterator() {
        assert_eq!(
            SAMPLE.lines().instructions().collect::<Vec<Instruction>>(),
            vec![Instruction::Noop, Instruction::Addx(3), Instruction::Addx(-5)]
            )
    }

    #[test]
    fn test_simulation() {
        assert_eq!(
            SAMPLE.lines().instructions().simulate().collect::<Vec<Registers>>(),
            vec![
                Registers {x: 1}, // noop
                Registers {x: 1}, // addx 3
                Registers {x: 1}, // -
                Registers {x: 4}, // addx -5
                Registers {x: 4}, // -
            ]
        );
    }

    #[test]
    fn test_signal_strength() {
        assert_eq!(calculate_signal_strength(LARGE_SAMPLE), 13140);
    }
}
