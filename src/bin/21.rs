use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

#[derive(Debug)]
enum NumKeypadButton {
    B7,
    B8,
    B9,
    B4,
    B5,
    B6,
    B1,
    B2,
    B3,
    B0,
    BA,
}

impl NumKeypadButton {
    fn from_char(char: char) -> Self {
        match char {
            '0' => NumKeypadButton::B0,
            '1' => NumKeypadButton::B1,
            '2' => NumKeypadButton::B2,
            '3' => NumKeypadButton::B3,
            '4' => NumKeypadButton::B4,
            '5' => NumKeypadButton::B5,
            '6' => NumKeypadButton::B6,
            '7' => NumKeypadButton::B7,
            '8' => NumKeypadButton::B8,
            '9' => NumKeypadButton::B9,
            'A' => NumKeypadButton::BA,
            _ => panic!(),
        }
    }

    fn get_row_col(self: &Self) -> (usize, usize) {
        let row = match self {
            Self::B7 | Self::B8 | Self::B9 => 0,
            Self::B4 | Self::B5 | Self::B6 => 1,
            Self::B1 | Self::B2 | Self::B3 => 2,
            Self::B0 | Self::BA => 3,
        };
        let col = match self {
            Self::B7 | Self::B4 | Self::B1 => 0,
            Self::B8 | Self::B5 | Self::B2 | Self::B0 => 1,
            Self::B9 | Self::B6 | Self::B3 | Self::BA => 2,
        };
        (row, col)
    }
}

#[derive(Debug)]
struct NumKeypadArm {
    row: usize,
    col: usize,
}

impl NumKeypadArm {
    fn new() -> Self {
        let (row, col) = NumKeypadButton::BA.get_row_col();
        Self { row, col }
    }

    fn moves_to(self: &mut Self, button: NumKeypadButton) -> Vec<ArmMove> {
        println!("{:?}.moves_to({:?})", self, button);
        let (row, col) = button.get_row_col();
        let horz_change = col as isize - self.col as isize;
        let vert_change = self.row as isize - row as isize;

        let horz_move = if horz_change > 0 {
            ArmMove::Right
        } else {
            ArmMove::Left
        };
        let mut horz_moves = vec![horz_move; horz_change.abs() as usize];

        let vert_move = if vert_change > 0 {
            ArmMove::Up
        } else {
            ArmMove::Down
        };
        let mut vert_moves = vec![vert_move; vert_change.abs() as usize];

        let mut move_collections = vec![];
        if vert_change > 0 {
            move_collections.append(&mut vert_moves);
            move_collections.append(&mut horz_moves);
        } else {
            move_collections.append(&mut horz_moves);
            move_collections.append(&mut vert_moves);
        }
        move_collections.push(ArmMove::APush);

        self.col = col;
        self.row = row;
        println!("\t{:?}", move_collections);

        move_collections
    }
}

#[derive(Clone, Copy)]
enum ArmMove {
    Up,
    Down,
    Left,
    Right,
    APush,
}

impl ArmMove {
    fn to_char(self: &Self) -> char {
        match self {
            ArmMove::Up => '^',
            ArmMove::Down => 'v',
            ArmMove::Left => '<',
            ArmMove::Right => '>',
            ArmMove::APush => 'A',
        }
    }
}

impl Debug for ArmMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug)]
enum DirKeypadButton {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl DirKeypadButton {
    fn from_move(arm_move: &ArmMove) -> Self {
        match arm_move {
            ArmMove::Up => Self::Up,
            ArmMove::Down => Self::Down,
            ArmMove::Left => Self::Left,
            ArmMove::Right => Self::Right,
            ArmMove::APush => Self::A,
        }
    }

    fn get_row_col(self: &Self) -> (usize, usize) {
        let row = match self {
            Self::Up | Self::A => 0,
            Self::Left | Self::Down | Self::Right => 1,
        };
        let col = match self {
            Self::Left => 0,
            Self::Up | Self::Down => 1,
            Self::A | Self::Right => 2,
        };
        (row, col)
    }
}

#[derive(Debug)]
struct DirKeypadArm {
    row: usize,
    col: usize,
}

impl DirKeypadArm {
    fn new() -> Self {
        let (row, col) = DirKeypadButton::A.get_row_col();
        Self { row, col }
    }

    fn moves_to(self: &mut Self, button: DirKeypadButton) -> Vec<ArmMove> {
        println!("{:?}.moves_to({:?})", self, button);

        let (row, col) = button.get_row_col();
        let horz_change = col as isize - self.col as isize;
        let vert_change = self.row as isize - row as isize;

        let horz_move = if horz_change > 0 {
            ArmMove::Right
        } else {
            ArmMove::Left
        };
        let mut horz_moves = vec![horz_move; horz_change.abs() as usize];

        let vert_move = if vert_change > 0 {
            ArmMove::Up
        } else {
            ArmMove::Down
        };
        let mut vert_moves = vec![vert_move; vert_change.abs() as usize];

        let mut move_collections = vec![];
        if vert_change > 0 {
            move_collections.append(&mut horz_moves);
            move_collections.append(&mut vert_moves);
        } else {
            move_collections.append(&mut vert_moves);
            move_collections.append(&mut horz_moves);
        }
        move_collections.push(ArmMove::APush);

        self.col = col;
        self.row = row;
        println!("\t{:?}", move_collections);

        move_collections
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| {
                let line = line.trim();
                let numeric_part = line[0..3].parse::<usize>().unwrap();

                // Arm pressing numeric keypad
                let mut num_keypad_arm = NumKeypadArm::new();
                let num_keypad_arm_moves = line
                    .chars()
                    .map(NumKeypadButton::from_char)
                    .flat_map(|button| num_keypad_arm.moves_to(button))
                    .collect_vec();

                println!(
                    "\"{}\": {}",
                    line,
                    num_keypad_arm_moves
                        .iter()
                        .map(|arm_move| arm_move.to_char())
                        .join(""),
                );

                // Second arm
                let mut intermediate_dir_keypad_arm = DirKeypadArm::new();
                let intermediate_dir_keypad_arm_moves = num_keypad_arm_moves
                    .iter()
                    .map(DirKeypadButton::from_move)
                    .flat_map(|button| intermediate_dir_keypad_arm.moves_to(button))
                    .collect_vec();

                println!(
                    "\"{}\": {}",
                    line,
                    intermediate_dir_keypad_arm_moves
                        .iter()
                        .map(|arm_move| arm_move.to_char())
                        .join(""),
                );

                // Arm we are controlling
                let mut controlled_dir_keypad_arm = DirKeypadArm::new();
                let controlled_dir_keypad_arm_moves = intermediate_dir_keypad_arm_moves
                    .iter()
                    .map(DirKeypadButton::from_move)
                    .flat_map(|button| controlled_dir_keypad_arm.moves_to(button))
                    .collect_vec();

                // Keypad for us to press

                let sequence_length: usize = controlled_dir_keypad_arm_moves.len();
                let complexity = numeric_part * sequence_length;

                println!(
                    "\"{}\": {} ({})",
                    line,
                    controlled_dir_keypad_arm_moves
                        .iter()
                        .map(|arm_move| arm_move.to_char())
                        .join(""),
                    complexity
                );

                complexity
            })
            .sum())
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

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
