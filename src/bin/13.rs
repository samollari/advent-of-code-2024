use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;
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

#[derive(Debug)]
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

fn find_possible_solutions(a_step: usize, b_step: usize, goal: usize) -> HashSet<(usize, usize)> {
    let max_a_step = min(goal / a_step, 100);

    (1..=max_a_step)
        .filter_map(|count_a| {
            let rem_for_b = goal - (count_a * a_step);
            let count_b = rem_for_b / b_step;
            let rem = rem_for_b - (count_b * b_step);

            if rem == 0 && count_b <= 100 {
                Some((count_a, count_b))
            } else {
                None
            }
        })
        .collect()
}

fn attempt_find_min_tokens_to_prize(machine: &Machine) -> Option<usize> {
    let x_solutions = find_possible_solutions(
        machine.a_button.x_step,
        machine.b_button.x_step,
        machine.prize_loc.x,
    );
    let y_solutions = find_possible_solutions(
        machine.a_button.y_step,
        machine.b_button.y_step,
        machine.prize_loc.y,
    );

    println!("{:?} {:?} {:?}", machine, x_solutions, y_solutions);

    x_solutions
        .intersection(&y_solutions)
        .map(|(a_presses, b_presses)| 3 * a_presses + b_presses)
        .sorted()
        .next()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let machines = parse_input(reader);
        // println!("{:?}", machines);

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
