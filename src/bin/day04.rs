use anyhow::Result;
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

const DIRECTIONS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

fn problem1_solution(input: &Vec<String>) -> Result<usize> {
    let mut total = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for d in DIRECTIONS.iter() {
                if char_at(input, y, x) == Some(b'X')
                    && char_at(
                        input,
                        y.wrapping_add_signed(d[0]),
                        x.wrapping_add_signed(d[1]),
                    ) == Some(b'M')
                    && char_at(
                        input,
                        y.wrapping_add_signed(2 * d[0]),
                        x.wrapping_add_signed(2 * d[1]),
                    ) == Some(b'A')
                    && char_at(
                        input,
                        y.wrapping_add_signed(3 * d[0]),
                        x.wrapping_add_signed(3 * d[1]),
                    ) == Some(b'S')
                {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

fn char_at(input: &Vec<String>, y: usize, x: usize) -> Option<u8> {
    let row = input.get(y)?;
    row.as_bytes().get(x).cloned()
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let mut total = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if char_at(input, y, x) == Some(b'A') {
                let ne1 = char_at(input, y.wrapping_add_signed(1), x.wrapping_add_signed(1))
                    == Some(b'M')
                    && char_at(input, y.wrapping_add_signed(-1), x.wrapping_add_signed(-1))
                        == Some(b'S');
                let ne2 = char_at(input, y.wrapping_add_signed(1), x.wrapping_add_signed(1))
                    == Some(b'S')
                    && char_at(input, y.wrapping_add_signed(-1), x.wrapping_add_signed(-1))
                        == Some(b'M');
                let se1 = char_at(input, y.wrapping_add_signed(-1), x.wrapping_add_signed(1))
                    == Some(b'M')
                    && char_at(input, y.wrapping_add_signed(1), x.wrapping_add_signed(-1))
                        == Some(b'S');
                let se2 = char_at(input, y.wrapping_add_signed(-1), x.wrapping_add_signed(1))
                    == Some(b'S')
                    && char_at(input, y.wrapping_add_signed(1), x.wrapping_add_signed(-1))
                        == Some(b'M');

                if (ne1 || ne2) && (se1 || se2) {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 18);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 9);
    }
}
