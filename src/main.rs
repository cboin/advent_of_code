use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn find_floor(inputs: &str) -> i32 {
    inputs.chars().fold(0, |mut acc, c| {
        if c == '(' {
            acc += 1
        } else {
            acc -= 1
        }
        acc
    })
}

fn main() {
    let inputs = lines_from_file("./input.txt");
    inputs
        .iter()
        .for_each(|input| println!("[part1]: {}", find_floor(input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_floor_zero_test() {
        assert_eq!(0, find_floor("(())"));
        assert_eq!(0, find_floor("()()"));
    }

    #[test]
    fn its_floor_three() {
        assert_eq!(3, find_floor("((("));
        assert_eq!(3, find_floor("(()(()("));
        assert_eq!(3, find_floor("))((((("));
    }

    #[test]
    fn its_floor_neg_one() {
        assert_eq!(-1, find_floor("())"));
        assert_eq!(-1, find_floor("))("));
    }

    #[test]
    fn its_floor_neg_three() {
        assert_eq!(-3, find_floor(")))"));
        assert_eq!(-3, find_floor(")())())"));
    }
}
