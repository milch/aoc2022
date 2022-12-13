const INPUT: &str = include_str!("day_08.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn map_tallest() -> impl FnMut(&u32) -> bool {
    let mut tallest_so_far = 0;
    let mut first = true;
    move |&tree| {
        if first {
            tallest_so_far = tree;
            first = false;
            true
        } else {
            if tree > tallest_so_far {
                tallest_so_far = tree;
                true
            } else {
                false
            }
        }
    }
}

fn map_visible_trees() -> impl FnMut(&u32) -> u32 {
    let mut seen = vec![];
    move |&tree| {
        let score = seen
            .iter()
            .rev()
            .enumerate()
            .find(|&(_, &seen_tree)| seen_tree >= tree) // Find the first taller tree
            .map(|(idx, _)| idx + 1) // Convert idx to count
            .unwrap_or(seen.len()); // We haven't seen any taller trees

        seen.push(tree);

        score as u32
    }
}

fn transpose<T: Copy>(row_wise: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_length = row_wise[0].len();
    let col_length = row_wise.len();
    let mut result: Vec<Vec<T>> = (0..row_length)
        .map(|_| vec![row_wise[0][0]; col_length])
        .collect();
    for col in 0..col_length {
        for row in 0..row_length {
            result[row][col] = row_wise[col][row];
        }
    }
    result
}

fn compute_directionally<F, Mapper, Combine, T>(
    trees: Vec<Vec<u32>>,
    mapper: Mapper,
    combine: Combine,
) -> Vec<Vec<T>>
where
    F: FnMut(&u32) -> T,
    Mapper: Fn() -> F,
    Combine: Fn(T, T, T, T) -> T,
    T: Copy,
{
    let forwards: Vec<Vec<T>> = trees
        .iter()
        .map(|line| line.iter().map(mapper()).collect())
        .collect();

    let backwards: Vec<Vec<T>> = trees
        .iter()
        .map(|line| {
            let mut visibility: Vec<T> = line.iter().rev().map(mapper()).collect();
            visibility.reverse();
            visibility
        })
        .collect();

    let from_top: Vec<Vec<T>> = transpose(&trees)
        .iter()
        .map(|line| line.iter().map(mapper()).collect())
        .collect();

    let from_bottom: Vec<Vec<T>> = transpose(&trees)
        .iter()
        .map(|line| {
            let mut visibility: Vec<T> = line.iter().rev().map(mapper()).collect();
            visibility.reverse();
            visibility
        })
        .collect();

    forwards
        .iter()
        .zip(backwards)
        .zip(transpose(&from_top))
        .zip(transpose(&from_bottom))
        .map(|(((l, r), u), d)| {
            l.iter()
                .zip(r)
                .zip(u)
                .zip(d)
                .map(|(((&w, x), y), z)| combine(w, x, y, z))
                .collect()
        })
        .collect()
}

fn compute_visibility(trees: Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    return compute_directionally(trees, map_tallest, |w, x, y, z| w || x || y || z);
}

fn compute_scenic_score(trees: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    return compute_directionally(trees, map_visible_trees, |w, x, y, z| w * x * y * z);
}

fn count_visible(trees: Vec<Vec<bool>>) -> usize {
    trees
        .iter()
        .map(|line| {
            line.iter()
                .map(|&tree| if tree { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn highest_score(scores: Vec<Vec<u32>>) -> u32 {
    *scores
        .iter()
        .map(|line| line.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(&0)
}

pub fn print_solution() {
    let trees = parse_input(INPUT);
    let visible_trees = compute_visibility(trees.clone());
    let visible_tree_count = count_visible(visible_trees);
    println!("Number of visible trees: {visible_tree_count}");
    let scenic_scores = compute_scenic_score(trees.clone());
    let highest_score = highest_score(scenic_scores);
    println!("Highest scenic score: {highest_score}");
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = "30373
                          25512
                          65332
                          33549
                          35390";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]
        );
    }

    #[test]
    fn test_compute_visibility() {
        assert_eq!(
            compute_visibility(parse_input(SAMPLE)),
            vec![
                vec![true, true, true, true, true],
                vec![true, true, true, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true],
            ]
        );
    }

    #[test]
    fn test_compute_scenic_score() {
        let scores = compute_scenic_score(parse_input(SAMPLE));
        assert_eq!(highest_score(scores), 8)
    }

    #[test]
    fn test_count_visible() {
        assert_eq!(count_visible(compute_visibility(parse_input(SAMPLE))), 21);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            transpose(&vec![vec![1, 2, 3], vec![4, 5, 6],]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }

    #[test]
    fn test_map_visible() {
        let trans = |v: Vec<u32>| v.iter().map(map_visible_trees()).collect::<Vec<u32>>();
        assert_eq!(trans(vec![3, 0, 3, 7, 3]), vec![0, 1, 2, 3, 1]);
        assert_eq!(trans(vec![2, 5, 5, 1, 2]), vec![0, 1, 1, 1, 2]);
        assert_eq!(trans(vec![6, 5, 3, 3, 2]), vec![0, 1, 1, 1, 1]);
        assert_eq!(trans(vec![3, 3, 5, 4, 9]), vec![0, 1, 2, 1, 4]);
        assert_eq!(trans(vec![3, 5, 3, 9, 0]), vec![0, 1, 1, 3, 1]);
    }
}
