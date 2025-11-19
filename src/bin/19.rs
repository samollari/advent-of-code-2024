use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

fn parse_input<R: BufRead>(reader: R) -> (Vec<String>, Vec<String>) {
    let mut lines = reader.lines();
    let available_patterns = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(", ")
        .map(|slice| slice.to_string())
        .collect();
    let _ = lines.next().unwrap().unwrap();
    let requested_patterns = lines.map(|line| line.unwrap()).collect();

    (available_patterns, requested_patterns)
}

fn try_fit_pattern(patterns: &Vec<String>, requested_pattern: &str) -> bool {
    if requested_pattern.len() == 0 {
        return true;
    }

    // println!("try_fit_pattern({:?}, \"{}\")", patterns, requested_pattern);

    patterns.iter().any(|pattern| {
        // println!("\t{}?", pattern);
        if pattern.len() > requested_pattern.len() {
            return false;
        }
        let pattern_match_slice = &requested_pattern[0..pattern.len()];
        let rest_slice = &requested_pattern[pattern.len()..];
        // println!("\t\tmatch: {}\trest: {}", pattern_match_slice, rest_slice);
        pattern_match_slice == *pattern && try_fit_pattern(patterns, rest_slice)
    })
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (available_patterns, requested_patterns) = parse_input(reader);

        Ok(requested_patterns
            .iter()
            .filter(|pattern| try_fit_pattern(&available_patterns, &pattern))
            .count())
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
