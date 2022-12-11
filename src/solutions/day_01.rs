const INPUT: &str = include_str!("day_01.txt");

fn convert_input_to_array<'a>(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .fold(vec![vec![]], |mut accum: Vec<Vec<i32>>, line: &str| {
            if line.is_empty() {
                accum.push(vec![]);
            } else {
                let last = accum.last_mut().unwrap();
                let current_value: i32 = line.parse().unwrap();
                last.push(current_value);
            }
            accum
        })
}

pub fn print_solution() {
    let result = convert_input_to_array(INPUT);
    let mut sums: Vec<i32> = result
        .iter()
        .map(|carrying| carrying.iter().fold(0, |sum, &i| sum + i))
        .collect();
    let single_max = sums
        .iter()
        .reduce(|max, i| if i > max { i } else { max })
        .unwrap();
    println!("Max: {:?}", single_max);
    sums.sort_unstable_by(|a, b| b.cmp(a));
    let highest_three = sums.as_slice().get(0..3).unwrap();
    println!("Max 3: {:?}", highest_three.iter().sum::<i32>())
}
