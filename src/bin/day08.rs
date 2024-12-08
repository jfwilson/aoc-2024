use anyhow::Result;
use itertools::Itertools;
use nalgebra::Vector2;
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

fn problem_solution(input: &Vec<String>, part2: bool) -> usize {
    let max_y = input.len();
    let max_x = input.iter().map(|line| line.len()).max().unwrap();
    let mut locations = Vec::new();
    for b in (b'0'..=b'9').chain(b'A'..=b'Z').chain(b'a'..=b'z') {
        let mut positions = Vec::new();
        for y in 0..input.len() {
            positions.extend(
                input[y]
                    .as_bytes()
                    .iter()
                    .positions(|&c| c == b)
                    .map(|x| Vector2::from([x as isize, y as isize])),
            );
        }
        for (a, b) in positions.iter().tuple_combinations() {
            let diff = b - a;
            if part2 {
                let mut p = a.clone();
                while is_in_range(&p, max_x, max_y) {
                    locations.push(p);
                    p -= diff;
                }
                p = b.clone();
                while is_in_range(&p, max_x, max_y) {
                    locations.push(p);
                    p += diff;
                }
            } else {
                locations.extend(
                    [a - diff, b + diff]
                        .into_iter()
                        .filter(|p| is_in_range(p, max_x, max_y)),
                );
            }
        }
    }
    locations.sort_by_key(|p| (p.y, p.x));
    locations.dedup();
    locations.len()
}

fn is_in_range(p: &Vector2<isize>, max_x: usize, max_y: usize) -> bool {
    (0..max_x).contains(&(p.x as usize)) && (0..max_y).contains(&(p.y as usize))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem_solution(&load_test_data(), false);
        assert_eq!(answer, 14);
    }

    #[test]
    fn problem2() {
        let input = &load_test_data();
        let answer = problem_solution(input, true);
        assert_eq!(answer, 34);
    }
}
