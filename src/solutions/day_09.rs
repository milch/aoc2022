use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::str::Lines;

const INPUT: &str = include_str!("day_09.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    count: u32,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RopePosition<const N: usize> {
    positions: [Position; N],
}

struct Rope<'a, const N: usize> {
    positions: [Position; N],
    instructions: &'a mut ExpandedInstructions<'a>,
}

impl<'a, const N: usize> Rope<'a, N> {
    fn new(instructions: &'a mut ExpandedInstructions<'a>) -> Rope<N> {
        Rope {
            positions: [Position { x: 0, y: 0 }; N],
            instructions,
        }
    }
    fn update_tail(&self, head_position: Position, mut tail_position: Position) -> Position {
        let diff_x = head_position.x - tail_position.x;
        let diff_y = head_position.y - tail_position.y;

        match (diff_x.abs(), diff_y.abs()) {
            (2, 0) => tail_position.x += diff_x.signum(),
            (0, 2) => tail_position.y += diff_y.signum(),
            (0, 0) => {} // Overlapping - nothing to do
            (1, 0) => {} // Touching - same row
            (1, 1) => {} // Touching - diagonally
            (0, 1) => {} // Touching - same column
            _ => {
                // Apart - move diagonally towards head
                tail_position.x += diff_x.signum();
                tail_position.y += diff_y.signum();
            }
        }

        tail_position
    }
}

impl<'a, const N: usize> Iterator for Rope<'a, N> {
    type Item = RopePosition<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(direction) = self.instructions.next() else {
            return None;
        };

        match direction {
            Direction::Up => self.positions[0].y += 1,
            Direction::Left => self.positions[0].x -= 1,
            Direction::Right => self.positions[0].x += 1,
            Direction::Down => self.positions[0].y -= 1,
        }

        for i in 1..N {
            self.positions[i] = self.update_tail(self.positions[i - 1], self.positions[i]);
        }

        Some(RopePosition {
            positions: self.positions,
        })
    }
}

struct Instructions<'a> {
    lines: &'a mut Lines<'a>,
}

impl<'a> Instructions<'a> {
    fn expand(&'a mut self) -> ExpandedInstructions {
        ExpandedInstructions {
            instructions: self,
            current_count: None,
            current_direction: None,
        }
    }
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(line) = self.lines.next() else {
            return None;
        };

        let mut parts = line.trim().split(" ");
        let direction = parts.next();
        let count = parts.next().map(|p| p.parse::<u32>().ok());

        let Some(direction) = direction else { return None; };
        let Some(Some(count)) = count else { return None; };

        let dir_enum = match direction {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!(),
        };

        Some(Instruction {
            count,
            direction: dir_enum,
        })
    }
}

struct ExpandedInstructions<'a> {
    instructions: &'a mut Instructions<'a>,
    current_count: Option<u32>,
    current_direction: Option<Direction>,
}

impl<'a> Iterator for ExpandedInstructions<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(count), Some(direction)) = (self.current_count, self.current_direction) {
            if count > 0 {
                self.current_count = Some(count - 1);
                return Some(direction);
            }
        }

        let Some(next_instruction) = self.instructions.next() else { return None; };

        self.current_count = Some(next_instruction.count - 1);
        self.current_direction = Some(next_instruction.direction);

        Some(next_instruction.direction)
    }
}

trait InstructionIterator<'a> {
    fn instructions(&'a mut self) -> Instructions;
}

impl<'a> InstructionIterator<'a> for Lines<'a> {
    fn instructions(&'a mut self) -> Instructions {
        Instructions { lines: self }
    }
}

impl<'a> ExpandedInstructions<'a> {
    fn rope<const N: usize>(&'a mut self) -> Rope<'a, N> {
        Rope::new(self)
    }
}

fn count_seen<const N: usize>(it: Rope<N>) -> usize {
    let set: HashSet<Position, RandomState> =
        HashSet::from_iter(it.map(|pos| pos.positions[N - 1]));
    set.len()
}

pub fn print_solution() {
    let small_rope_seen = count_seen(INPUT.lines().instructions().expand().rope::<2>());
    println!("Seen positions for rope with 2 knots: {small_rope_seen}");
    let large_rope_seen = count_seen(INPUT.lines().instructions().expand().rope::<10>());
    println!("Seen positions for rope with 10 knots: {large_rope_seen}");
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "R 4
                          U 4
                          L 3
                          D 1
                          R 4
                          D 1
                          L 5
                          R 2";

    const SHORT_SAMPLE: &str = "R 4\nU 3";

    const LARGE_SAMPLE: &str = "R 5
                                U 8
                                L 8
                                D 3
                                R 17
                                D 10
                                L 25
                                U 20";

    #[test]
    fn test_instructions_iterator() {
        assert_eq!(
            SAMPLE.lines().instructions().collect::<Vec<Instruction>>(),
            vec![
                Instruction {
                    count: 4,
                    direction: Direction::Right
                },
                Instruction {
                    count: 4,
                    direction: Direction::Up
                },
                Instruction {
                    count: 3,
                    direction: Direction::Left
                },
                Instruction {
                    count: 1,
                    direction: Direction::Down
                },
                Instruction {
                    count: 4,
                    direction: Direction::Right
                },
                Instruction {
                    count: 1,
                    direction: Direction::Down
                },
                Instruction {
                    count: 5,
                    direction: Direction::Left
                },
                Instruction {
                    count: 2,
                    direction: Direction::Right
                },
            ]
        )
    }

    #[test]
    fn test_expanded_instructions_iterator() {
        assert_eq!(
            SHORT_SAMPLE
                .lines()
                .instructions()
                .expand()
                .collect::<Vec<Direction>>(),
            vec![
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Up,
                Direction::Up,
                Direction::Up,
            ]
        );
    }

    #[test]
    fn test_rope_iterator() {
        assert_eq!(
            SHORT_SAMPLE
                .lines()
                .instructions()
                .expand()
                .rope::<2>()
                .last(),
            Some(RopePosition {
                positions: [Position { x: 4, y: 3 }, Position { x: 4, y: 2 }]
            })
        );
        assert_eq!(
            SAMPLE.lines().instructions().expand().rope::<2>().last(),
            Some(RopePosition {
                positions: [Position { x: 2, y: 2 }, Position { x: 1, y: 2 }]
            })
        );
    }

    #[test]
    fn test_count_seen() {
        assert_eq!(
            count_seen(SAMPLE.lines().instructions().expand().rope::<2>()),
            13
        );

        assert_eq!(
            count_seen(SAMPLE.lines().instructions().expand().rope::<10>()),
            1
        );

        assert_eq!(
            count_seen(LARGE_SAMPLE.lines().instructions().expand().rope::<10>()),
            36
        );
    }
}
