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

    println!("problem1 = {}", problem1_solution(&lines)?);
    println!("problem2 = {}", problem2_solution(&lines)?);
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> Result<usize> {
    let ns = parse_input(input);
    Ok(ns.into_iter().filter(|row| is_safe(row)).count())
}

fn is_safe(row: &Vec<isize>) -> bool {
    row.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(b - a))) ||
        row.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(a - b)))
}

fn is_subset_safe(row: &Vec<isize>, temp: &mut Vec<isize>) -> bool {
    for i in 0..row.len() {
        temp.clear();
        temp.extend_from_slice(&row[0..i]);
        temp.extend_from_slice(&row[(i + 1)..row.len()]);
        if is_safe(&temp) {
            return true;
        }
    }
    false
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let ns = parse_input(input);
    let mut temp = Vec::new();
    Ok(ns
        .into_iter()
        .filter(|row| is_safe(row) || is_subset_safe(row, &mut temp))
        .count())
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<isize>> {
    input
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 2);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 4);
    }
}
