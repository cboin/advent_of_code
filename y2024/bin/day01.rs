use itertools::Itertools;

fn main() {
    let inputs = include_str!("../inputs/day01.txt")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = inputs
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Invalid number in input"))
                .collect_tuple::<(i32, i32)>()
                .expect("Expected exactly two elements per line")
        })
        .unzip();

    left.sort();
    right.sort();

    let total_distance: i32 = left.iter().zip(&right).map(|(x, y)| (y - x).abs()).sum();

    println!("Total distance: {}", total_distance);

    let similarity_score: i32 = left
        .iter()
        .filter(|&&x| right.contains(&x))
        .map(|&x| x * right.iter().filter(|&&y| y == x).count() as i32)
        .sum();

    println!("Similarity score: {}", similarity_score);
}
