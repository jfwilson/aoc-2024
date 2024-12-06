use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
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
    let (pairs, updates) = parse_input(input);

    Ok(updates
        .into_iter()
        .filter(|update| is_ordered(&pairs, update))
        .map(|update| update[update.len() >> 1].parse::<usize>().unwrap())
        .sum())
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let (pairs, updates) = parse_input(input);

    Ok(updates
        .into_iter()
        .filter(|update| !is_ordered(&pairs, update))
        .map(|update| sort(&pairs, update))
        .map(|update| update[update.len() >> 1].parse::<usize>().unwrap())
        .sum())
}

fn parse_input(input: &Vec<String>) -> (HashMap<(&str, &str), usize>, Vec<Vec<&str>>) {
    let mut lines = input.iter();
    let pairs = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .counts();
    let updates = lines
        .map(|line| line.split(',').collect_vec())
        .collect_vec();
    (pairs, updates)
}

fn sort<'a>(pairs: &HashMap<(&str, &str), usize>, mut update: Vec<&'a str>) -> Vec<&'a str> {
    let mut counts = update.iter().cloned().counts();
    for (a, b) in update.iter().tuple_combinations() {
        if pairs.contains_key(&(a, b)) {
            *counts.entry(b).or_insert(0) += 1;
        }
        if pairs.contains_key(&(b, a)) {
            *counts.entry(a).or_insert(0) += 1;
        }
    }
    update.sort_by_key(|x| counts.get(x).unwrap_or(&0));
    update
}

fn is_ordered(pairs: &HashMap<(&str, &str), usize>, update: &Vec<&str>) -> bool {
    !update
        .iter()
        .tuple_combinations()
        .any(|(a, b)| pairs.contains_key(&(b, a)))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 143);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 123);
    }
}
