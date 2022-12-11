const INPUT: &str = include_str!("day_02.txt");

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn outcome(&self, other: &Hand) -> Outcome {
        match self {
            Self::Rock => match other {
                Hand::Rock => Outcome::Draw,
                Hand::Paper => Outcome::Lose,
                Hand::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Hand::Rock => Outcome::Win,
                Hand::Paper => Outcome::Draw,
                Hand::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Hand::Rock => Outcome::Lose,
                Hand::Paper => Outcome::Win,
                Hand::Scissors => Outcome::Draw,
            },
        }
    }

    fn hand_to_reach_outcome(&self, outcome: &Outcome) -> &Hand {
        [Self::Rock, Self::Paper, Self::Scissors]
            .iter()
            .find(|other| other.outcome(self) == *outcome)
            .unwrap()
    }

    fn from_char(ch: &str) -> Hand {
        match ch {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Unsupported char"),
        }
    }

    fn individual_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Lose,
    Win,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_char(ch: &str) -> Outcome {
        match ch {
            "X" => Outcome::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unsupported char!"),
        }
    }
}

fn convert_to_hands(input: &str) -> Vec<Vec<Hand>> {
    input
        .lines()
        .map(|line| line.split(' ').map(Hand::from_char).collect())
        .collect()
}

fn convert_to_hand_and_outcome(input: &str) -> Vec<(Hand, Outcome)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let hand = parts.first().unwrap();
            let outcome = parts.last().unwrap();
            (Hand::from_char(hand), Outcome::from_char(outcome))
        })
        .collect()
}

fn score_hands(hands: Vec<Vec<Hand>>) -> u32 {
    hands
        .iter()
        .map(|hands| {
            let other = hands.first().unwrap();
            let me = hands.last().unwrap();
            me.outcome(other).score() + me.individual_score()
        })
        .sum()
}

fn find_hands_to_reach_outcome(input: Vec<(Hand, Outcome)>) -> Vec<Vec<Hand>> {
    input
        .iter()
        .map(|(hand, outcome)| {
            let my_hand = hand.hand_to_reach_outcome(outcome);
            vec![*hand, *my_hand]
        })
        .collect()
}

pub fn print_solution() {
    let hands_map = convert_to_hands(INPUT);
    let total_score: u32 = score_hands(hands_map);
    println!("Total score first part: {:?}", total_score);

    let hands_and_desired_outcomes = convert_to_hand_and_outcome(INPUT);
    let hands_from_outcomes: Vec<Vec<Hand>> =
        find_hands_to_reach_outcome(hands_and_desired_outcomes);
    let total_score_outcomes = score_hands(hands_from_outcomes);
    println!("Total score second part: {}", total_score_outcomes)
}

#[cfg(test)]
mod test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z
";

    #[test]
    fn test_first_part_output() {
        assert_eq!(score_hands(convert_to_hands(SAMPLE)), 15)
    }

    #[test]
    fn test_second_part_output() {
        assert_eq!(
            score_hands(find_hands_to_reach_outcome(convert_to_hand_and_outcome(
                SAMPLE
            ))),
            12
        )
    }
}
