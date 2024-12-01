use anyhow::{anyhow, Result};
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
    let (mut lhs, mut rhs) = parse_input(input)?;
    lhs.sort_unstable();
    rhs.sort_unstable();
    Ok(lhs
        .into_iter()
        .zip(rhs.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let (lhs, rhs) = parse_input(input)?;
    let counts = rhs.into_iter().counts();
    Ok(lhs
        .into_iter()
        .map(|l| l * counts.get(&l).unwrap_or(&0))
        .sum())
}

fn parse_input(input: &Vec<String>) -> Result<(Vec<usize>, Vec<usize>)> {
    let mut lhs = Vec::with_capacity(input.len());
    let mut rhs = Vec::with_capacity(input.len());
    for line in input {
        let (l, r) = line
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(|| anyhow!("Unexpected input"))?;
        lhs.push(l.parse::<usize>()?);
        rhs.push(r.parse::<usize>()?);
    }
    Ok((lhs, rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 11);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 31);
    }
}
