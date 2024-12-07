use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;

    println!("problem1 = {}", problem_solution(&lines, false));
    println!("problem2 = {}", problem_solution(&lines, true));
    Ok(())
}

fn problem_solution(input: &Vec<String>, allow_concat: bool) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let (ts, rs) = line.split_once(": ").unwrap();
            let target = ts.parse::<usize>().unwrap();
            let ns = rs
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            is_possible(target, ns[0], &ns[1..], allow_concat).then_some(target)
        })
        .sum()
}

fn is_possible(target: usize, acc: usize, ns: &[usize], allow_concat: bool) -> bool {
    if ns.is_empty() {
        target == acc
    } else if acc > target {
        false
    } else {
        let n = ns[0];
        is_possible(target, acc + n, &ns[1..], allow_concat)
            || is_possible(target, acc * n, &ns[1..], allow_concat)
            || (allow_concat && is_possible(target, concat(acc, n), &ns[1..], allow_concat))
    }
}

fn concat(prefix: usize, suffix: usize) -> usize {
    prefix * 10usize.pow(1 + suffix.ilog10()) + suffix
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn concatenate() {
        assert_eq!(concat(12, 1), 121);
        assert_eq!(concat(12, 9), 129);
        assert_eq!(concat(12, 345), 12345);
    }

    #[test]
    fn problem1() {
        let answer = problem_solution(&load_test_data(), false);
        assert_eq!(answer, 3749);
    }

    #[test]
    fn problem2() {
        let answer = problem_solution(&load_test_data(), true);
        assert_eq!(answer, 11387);
    }
}
