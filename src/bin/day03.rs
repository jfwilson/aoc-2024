use anyhow::Result;
use regex::Regex;
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

fn problem1_solution(input: &Vec<String>) -> Result<isize> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total = 0;
    for line in input {
        for capture in re.captures_iter(&line) {
            let x = capture[1].parse::<isize>()?;
            let y = capture[2].parse::<isize>()?;
            total += x * y
        }
    }
    Ok(total)
}

fn problem2_solution(input: &Vec<String>) -> Result<isize> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")?;

    let mut total = 0;
    let mut enabled = true;
    for line in input {
        for capture in re.captures_iter(&line) {
            if capture[0].starts_with("don't") {
                enabled = false;
            } else if capture[0].starts_with("do") {
                enabled = true;
            } else if enabled {
                let x = capture[2].parse::<isize>()?;
                let y = capture[3].parse::<isize>()?;
                total += x * y
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    fn load_test_data(input: &str) -> Vec<String> {
        input.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data(INPUT1)).unwrap();
        assert_eq!(answer, 161);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data(INPUT2)).unwrap();
        assert_eq!(answer, 48);
    }
}
