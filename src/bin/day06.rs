use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
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

    println!(
        "problem1 = {}",
        path_length(&lines, None).unwrap_or_default()
    );
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

const DIRECTIONS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

const DIRECTION_CHARS: [char; 4] = ['>', 'v', '<', '^'];

fn path_length(input: &Vec<String>, obstacle: Option<(usize, usize)>) -> Option<usize> {
    let (mut y, start_line) = input
        .iter()
        .find_position(|line| line.contains(&DIRECTION_CHARS))
        .unwrap();
    let mut x = start_line.find(&DIRECTION_CHARS).unwrap();
    let mut d = DIRECTION_CHARS
        .iter()
        .position(|c| start_line.chars().contains(c))
        .unwrap();
    let max_x = input.iter().map(|line| line.len()).max().unwrap();
    let mut visited = HashSet::new();
    let mut states = HashSet::new();
    loop {
        if !states.insert((x, y, d)) {
            return None;
        }
        visited.insert((x, y));
        let next_x = x.wrapping_add_signed(DIRECTIONS[d][0]);
        let next_y = y.wrapping_add_signed(DIRECTIONS[d][1]);
        if next_y >= input.len() || next_x >= max_x {
            return Some(visited.len());
        }
        if &input[next_y][next_x..=next_x] == "#" || obstacle == Some((next_x, next_y)) {
            d = (d + 1) & 3;
        } else {
            x = next_x;
            y = next_y;
        }
    }
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let mut count = 0;
    for (obstacle_y, line) in input.iter().enumerate() {
        for obstacle_x in 0..line.len() {
            if &line[obstacle_x..=obstacle_x] == "."
                && path_length(input, Some((obstacle_x, obstacle_y))).is_none()
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = path_length(&load_test_data(), None).unwrap();
        assert_eq!(answer, 41);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 6);
    }
}
