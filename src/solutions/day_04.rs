const INPUT: &str = include_str!("day_04.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(range: std::ops::Range<u32>) -> Range {
        Range {
            start: range.start,
            end: range.end,
        }
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }

    fn contains(&self, num: u32) -> bool {
        self.start <= num && self.end >= num
    }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut ranges = line.trim().split(",").map(|assignment| {
                let mut parts = assignment.split("-");
                let start: u32 = parts.next().unwrap().parse().unwrap();
                let end: u32 = parts.next().unwrap().parse().unwrap();
                Range::new(start..end)
            });
            let lhs = ranges.next().unwrap();
            let rhs = ranges.next().unwrap();
            (lhs, rhs)
        })
        .collect()
}

fn count_fully_contained_pairs(pairs: Vec<(Range, Range)>) -> usize {
    pairs
        .iter()
        .filter(|(lhs, rhs)| lhs.fully_contains(rhs) || rhs.fully_contains(lhs))
        .count()
}

fn count_overlaps(pairs: Vec<(Range, Range)>) -> usize {
    pairs
        .iter()
        .filter(|(lhs, rhs)| lhs.overlaps(rhs) || rhs.overlaps(lhs))
        .count()
}

pub fn print_solution() {
    let number_of_double_shifts = count_fully_contained_pairs(parse_input(INPUT));
    println!("Number of double shifts: {number_of_double_shifts}");

    let number_of_overlaps = count_overlaps(parse_input(INPUT));
    println!("Number of overlaps: {number_of_overlaps}")
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn test_fully_contains() {
        assert!(Range::new(2..8).fully_contains(&Range::new(3..7)));
        assert!(Range::new(4..6).fully_contains(&Range::new(6..6)));
        assert!(Range::new(4..6).fully_contains(&Range::new(4..4)));
        assert!(Range::new(6..6).fully_contains(&Range::new(6..6)));
    }

    #[test]
    fn test_contains() {
        assert!(Range::new(2..8).contains(2));
        assert!(Range::new(2..8).contains(3));
        assert!(Range::new(2..8).contains(7));
        assert!(Range::new(2..8).contains(8));
        assert!(!Range::new(2..8).contains(1));
        assert!(!Range::new(2..8).contains(9));
    }

    #[test]
    fn test_overlaps() {
        assert!(!Range::new(2..4).overlaps(&Range::new(6..8)));
        assert!(!Range::new(2..3).overlaps(&Range::new(4..5)));
        assert!(Range::new(5..7).overlaps(&Range::new(7..9)));
        assert!(Range::new(2..8).overlaps(&Range::new(3..7)));
        assert!(Range::new(6..6).overlaps(&Range::new(4..6)));
        assert!(Range::new(2..6).overlaps(&Range::new(4..8)));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE),
            vec![
                (Range::new(2..4), Range::new(6..8)),
                (Range::new(2..3), Range::new(4..5)),
                (Range::new(5..7), Range::new(7..9)),
                (Range::new(2..8), Range::new(3..7)),
                (Range::new(6..6), Range::new(4..6)),
                (Range::new(2..6), Range::new(4..8)),
            ]
        )
    }

    #[test]
    fn test_count_pairs() {
        assert_eq!(count_fully_contained_pairs(parse_input(SAMPLE)), 2)
    }

    #[test]
    fn test_count_overlaps() {
        assert_eq!(count_overlaps(parse_input(SAMPLE)), 4)
    }
}
