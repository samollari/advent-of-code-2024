use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn extract_xy_parts_from_line(line: String) -> Option<(usize, usize)> {
    let (_, xy_part) = line.trim().split_once(": ")?;

    let (x_part, y_part) = xy_part.split_once(", ")?;

    return Some((
        x_part[2..].parse::<usize>().ok()?,
        y_part[2..].parse::<usize>().ok()?,
    ));
}

#[derive(Debug, Clone)]
struct Button {
    x_step: usize,
    y_step: usize,
}

impl Button {
    fn from_line(line: String) -> Self {
        match extract_xy_parts_from_line(line) {
            Some((x_step, y_step)) => Self { x_step, y_step },
            None => panic!(),
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from_line(line: String) -> Self {
        match extract_xy_parts_from_line(line) {
            Some((x, y)) => Self { x, y },
            None => panic!(),
        }
    }
}

#[derive(Debug)]
struct Machine {
    a_button: Button,
    b_button: Button,
    prize_loc: Coord,
}

impl Machine {
    fn from_chunk<T: BufRead>(mut chunk: itertools::Chunk<'_, Lines<T>>) -> Self {
        let first = chunk.next().unwrap().unwrap();
        let second = chunk.next().unwrap().unwrap();
        let third = chunk.next().unwrap().unwrap();

        Self {
            a_button: Button::from_line(first),
            b_button: Button::from_line(second),
            prize_loc: Coord::from_line(third),
        }
    }
}

fn parse_input<T: BufRead>(reader: T) -> Vec<Machine> {
    (&reader.lines().chunks(4))
        .into_iter()
        .map(Machine::from_chunk)
        .collect_vec()
}

fn attempt_find_min_tokens_to_prize(machine: &Machine) -> Option<usize> {
    let a_press_numerator = (machine.prize_loc.x * machine.b_button.y_step) as isize
        - (machine.prize_loc.y * machine.b_button.x_step) as isize;

    let a_press_denominator = (machine.a_button.x_step * machine.b_button.y_step) as isize
        - (machine.a_button.y_step * machine.b_button.x_step) as isize;

    if (a_press_numerator < 0) ^ (a_press_denominator < 0) {
        return None;
    }

    if a_press_numerator % a_press_denominator != 0 {
        return None;
    }
    let a_presses = a_press_numerator / a_press_denominator;
    assert!(a_presses >= 0);
    let a_presses = a_presses as usize;

    let b_press_numerator =
        machine.prize_loc.x as isize - (a_presses * machine.a_button.x_step) as isize;

    if b_press_numerator < 0 {
        return None;
    }
    let b_press_numerator = b_press_numerator as usize;

    if b_press_numerator % machine.b_button.x_step != 0 {
        return None;
    }
    let b_presses = b_press_numerator / machine.b_button.x_step;

    let tokens = 3 * a_presses + b_presses;

    return Some(tokens);
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = parse_input(reader);

        Ok(machines
            .iter()
            .filter_map(attempt_find_min_tokens_to_prize)
            .sum())
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let machines = parse_input(reader)
            .iter()
            .map(
                |Machine {
                     a_button,
                     b_button,
                     prize_loc,
                 }| Machine {
                    a_button: a_button.clone(),
                    b_button: b_button.clone(),
                    prize_loc: Coord {
                        x: prize_loc.x + 10000000000000,
                        y: prize_loc.y + 10000000000000,
                    },
                },
            )
            .collect_vec();

        Ok(machines
            .iter()
            .filter_map(attempt_find_min_tokens_to_prize)
            .sum())
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
