const INPUT: &str = include_str!("input.txt");

fn split_compartments(input: &str) -> Vec<[&str; 2]> {
    input
        .lines()
        .map(|line| {
            let first_part = &line[0..line.len() / 2];
            let second_part = &line[line.len() / 2..line.len()];
            [first_part, second_part]
        })
        .collect()
}

fn present_in_both(compartments: &[&str; 2]) -> char {
    let lhs = compartments[0];
    let rhs = compartments[1];
    lhs.chars()
        .find(|char| rhs.chars().any(|other| other == *char))
        .unwrap()
}

fn char_value(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        (ch as u32) + 1 - ('a' as u32)
    } else {
        (ch as u32) + 27 - ('A' as u32)
    }
}

fn priority_sum(input: &str) -> u32 {
    let compartments = split_compartments(input);
    compartments
        .iter()
        .map(|part| present_in_both(part))
        .fold(0, |sum, char| sum + char_value(char))
}

fn main() {
    let sum = priority_sum(INPUT);
    println!("Sum: {sum}")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_first_sample() {
        assert_eq!(priority_sum(SAMPLE), 157)
    }

    #[test]
    fn test_splitting_compartments() {
        let all_backpacks = split_compartments(SAMPLE);
        assert_eq!(
            all_backpacks[0].to_vec(),
            vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"]
        );
        assert_eq!(
            all_backpacks[1].to_vec(),
            vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]
        );
        assert_eq!(all_backpacks[2].to_vec(), vec!["PmmdzqPrV", "vPwwTWBwg"]);
    }

    #[test]
    fn test_present_in_both() {
        assert_eq!(present_in_both(&["vJrwpWtwJgWr", "hcsFMMfFFhFp"]), 'p');
        assert_eq!(
            present_in_both(&["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]),
            'L'
        );
        assert_eq!(present_in_both(&["PmmdzqPrV", "vPwwTWBwg"]), 'P');
    }

    #[test]
    fn test_char_value() {
        assert_eq!(char_value('a'), 1);
        assert_eq!(char_value('A'), 27);
    }
}
