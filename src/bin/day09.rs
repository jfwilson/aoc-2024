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

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> usize {
    let mut blocks = Vec::new();
    let mut is_gap: bool = false;
    let mut i: usize = 0;
    for b in input[0].as_bytes() {
        let len = b - b'0';
        let block = if is_gap {
            i += 1;
            None
        } else {
            Some(i)
        };
        blocks.extend((0..len).map(|_| block));
        is_gap = !is_gap;
    }
    i = blocks.len() - 1;
    let mut gap_index = 0;
    while gap_index < i {
        if blocks[gap_index].is_some() {
            gap_index += 1;
        } else {
            blocks[gap_index] = blocks.remove(i);
            i = blocks
                .iter()
                .enumerate()
                .rev()
                .find_map(|(ii, cc)| cc.map(|_| ii))
                .unwrap_or_default();
        }
    }
    blocks
        .iter()
        .map_while(|&block| block)
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    let mut is_gap: bool = false;
    let mut i: usize = 0;
    for b in input[0].as_bytes() {
        let len = (b - b'0') as usize;
        let range = (i, len);
        i += len;
        if is_gap {
            gaps.push(range);
        } else {
            files.push(range);
        };
        is_gap = !is_gap;
    }
    for f in files.iter_mut().rev() {
        if let Some(gap_index) = gaps.iter().position(|gap| gap.1 >= f.1) {
            let gap = &mut gaps[gap_index];
            if gap.0 < f.0 {
                f.0 = gap.0;
                gap.0 += f.1;
                gap.1 -= f.1;
            }
        }
    }
    files
        .iter()
        .enumerate()
        .flat_map(|(id, &f)| (f.0..(f.0 + f.1)).map(move |i| i * id))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "2333133121414131402";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 1928);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 2858);
    }
}
