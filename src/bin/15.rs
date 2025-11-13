use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::hash::RandomState;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct GPSCoordinate {
    value: usize,
}

impl GPSCoordinate {
    fn from_xy(x: usize, y: usize) -> GPSCoordinate {
        Self { value: y * 100 + x }
    }

    fn after_move(self: &Self, move_type: &Move) -> Self {
        let Self { value } = self;
        match move_type {
            Move::Up => Self { value: value - 100 },
            Move::Down => Self { value: value + 100 },
            Move::Left => Self { value: value - 1 },
            Move::Right => Self { value: value + 1 },
        }
    }

    fn _to_xy(self: &Self) -> (usize, usize) {
        let Self { value } = self;
        (value % 100, value / 100)
    }
}

#[derive(Debug)]
enum Entity {
    Wall,
    Box,
    Robot,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input<R: BufRead>(
    reader: R,
) -> (HashMap<GPSCoordinate, Entity>, GPSCoordinate, Vec<Move>) {
    let lines = reader
        .lines()
        .filter_map(|result| match result {
            Result::Ok(val) => Some(val),
            Err(_) => None,
        })
        .collect_vec();
    let (map_lines, move_lines) = lines.split_at(
        lines
            .iter()
            .find_position(|line| line.is_empty())
            .unwrap()
            .0,
    );

    // let mut robot_coordinate = None;
    let map_entries: HashMap<GPSCoordinate, Entity, RandomState> = HashMap::from_iter(
        map_lines
            .iter()
            .enumerate()
            .flat_map(|(line_number, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(char_number, chr)| {
                        let coordinate = GPSCoordinate::from_xy(char_number, line_number);
                        let entity_result = match chr {
                            '#' => Some(Entity::Wall),
                            'O' => Some(Entity::Box),
                            '@' => {
                                // robot_coordinate = Some(coordinate);
                                Some(Entity::Robot)
                            }
                            _ => None,
                        };
                        match entity_result {
                            Some(entity) => Some((coordinate, entity)),
                            None => None,
                        }
                    })
            }),
    );
    let robot_coordinate = map_entries
        .iter()
        .find(|(_, entity)| match entity {
            Entity::Robot => true,
            _ => false,
        })
        .map(|(coord, _)| *coord);
    let moves = move_lines
        .iter()
        .flat_map(|line| {
            line.trim().chars().map(|chr| match chr {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("Unexpected move character"),
            })
        })
        .collect_vec();

    // println!("{:?}\n{:?}\n{:?}", map_entries, robot_coordinate, moves);

    (map_entries, robot_coordinate.unwrap(), moves)
}

fn try_move(
    map: &mut HashMap<GPSCoordinate, Entity>,
    coordinate: &GPSCoordinate,
    mov: &Move,
) -> Option<GPSCoordinate> {
    let new_coordinate = coordinate.after_move(&mov);

    let can_move = match map.get(&new_coordinate) {
        Some(entity) => match entity {
            Entity::Wall => false,
            Entity::Box => try_move(map, &new_coordinate, mov).is_some(),
            Entity::Robot => panic!("try_move found a robot entity at a new location!"),
        },
        None => true,
    };

    match can_move {
        true => {
            let this_entity = map.remove(coordinate).unwrap();
            assert!(map.insert(new_coordinate, this_entity).is_none());
            Some(new_coordinate)
        }
        false => None,
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut map, mut robot_coordinate, moves) = parse_input(reader);

        for mov in moves {
            match try_move(&mut map, &robot_coordinate, &mov) {
                Some(new_coord) => robot_coordinate = new_coord,
                None => {}
            }
        }

        Ok(map
            .iter()
            .filter_map(|(coordinate, entity)| match entity {
                Entity::Box => Some(coordinate.value),
                _ => None,
            })
            .sum())
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

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
