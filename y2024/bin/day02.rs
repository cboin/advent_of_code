use itertools::Itertools;

fn main() {
    let inputs = include_str!("../inputs/day02.txt")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<isize>().expect("Invalid number in input"))
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<_>>();

    println!("(Part 1) Total safe reports: {}", part1(&inputs));
    println!("(Part 2) Total safe reports: {}", part2(&inputs));
}

fn is_safe(report: &[isize]) -> bool {
    let is_valid_diff = |a: isize, b: isize| (1..=3).contains(&(a - b).abs());

    report
        .iter()
        .tuple_windows()
        .all(|(&a, &b)| b > a && is_valid_diff(b, a))
        || report
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| a > b && is_valid_diff(a, b))
}

fn part1(reports: &[Vec<isize>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part2(reports: &[Vec<isize>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            if is_safe(report) {
                true
            } else {
                (0..report.len()).any(|i| {
                    let mut tmp_report = report.to_vec();
                    tmp_report.remove(i);
                    is_safe(&tmp_report)
                })
            }
        })
        .count()
}
