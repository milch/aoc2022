use std::{collections::HashSet, hash::Hash, str::Chars};

const INPUT: &str = include_str!("day_06.txt");

fn next_chunk(chars: &mut Chars, count: usize) -> Vec<char> {
    let mut result = vec![];
    for _ in 0..count {
        result.push(chars.next().unwrap());
    }
    result
}

fn all_unique<T: Eq + Hash>(vec: &Vec<T>) -> bool {
    let mut set = HashSet::new();
    for ch in vec {
        set.insert(ch);
    }
    set.len() == vec.len()
}

fn index_for_unique_chars(in_str: &str, count: usize) -> usize {
    let mut chars = in_str.chars();
    let mut last_chunk = next_chunk(&mut chars, count - 1);
    let all_until_unique: Vec<char> = chars
        .take_while(|next| {
            last_chunk.push(*next);
            if last_chunk.len() > count {
                last_chunk.remove(0);
            }
            !all_unique(&last_chunk)
        })
        .collect();
    count + all_until_unique.len()
}

pub fn print_solution() {
    println!(
        "Start of packet sequence: {}",
        index_for_unique_chars(INPUT, 4)
    );
    println!(
        "Start of message sequence: {}",
        index_for_unique_chars(INPUT, 14)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_first_four_unique() {
        assert_eq!(
            index_for_unique_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            7
        );
        assert_eq!(index_for_unique_chars("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(index_for_unique_chars("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            index_for_unique_chars("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            index_for_unique_chars("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }

    #[test]
    fn test_first_fourteen_unique() {
        assert_eq!(
            index_for_unique_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
    }

    #[test]
    fn test_all_unique() {
        assert!(all_unique(&vec![1, 2, 3]));
        assert!(all_unique(&vec![3, 4, 5]));
        assert!(!all_unique(&vec![3, 3, 4, 5]));
    }
}
