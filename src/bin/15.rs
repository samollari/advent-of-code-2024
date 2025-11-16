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

// const TEST: &str = "\
// ##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
// ";

const TEST: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
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

    fn to_xy(self: &Self) -> (usize, usize) {
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

    if !can_move {
        return None;
    }

    let this_entity = map.remove(coordinate).unwrap();
    assert!(map.insert(new_coordinate, this_entity).is_none());
    Some(new_coordinate)
}

#[derive(Clone, Copy)]
enum Part2Entity {
    Wall,
    Robot,
    DoubleWideBox(usize),
}

#[derive(Clone, Copy)]
struct DoubleWideBox {
    left: GPSCoordinate,
    right: GPSCoordinate,
}

fn can_move(
    map: &HashMap<GPSCoordinate, Part2Entity>,
    boxes: &Vec<DoubleWideBox>,
    coordinate: &GPSCoordinate,
    mov: &Move,
) -> bool {
    let mut coords_to_check = vec![coordinate.after_move(&mov)];
    while !coords_to_check.is_empty() {
        // println!("{:?}", coords_to_check);
        let coord = coords_to_check.pop().unwrap();

        match map.get(&coord) {
            Some(entity) => match entity {
                Part2Entity::Wall => return false,
                Part2Entity::Robot => panic!("Found a robot in can_move"),
                Part2Entity::DoubleWideBox(box_index) => match boxes.get(*box_index) {
                    Some(DoubleWideBox { left, right }) => match mov {
                        Move::Up | Move::Down => {
                            coords_to_check.push(left.after_move(&mov));
                            coords_to_check.push(right.after_move(&mov));
                        }
                        Move::Left => {
                            coords_to_check.push(left.after_move(&mov));
                        }
                        Move::Right => {
                            coords_to_check.push(right.after_move(&mov));
                        }
                    },
                    None => panic!("Could not find specified box"),
                },
            },
            None => {}
        }
    }

    true
}

fn try_doublewide_move(
    map: &mut HashMap<GPSCoordinate, Part2Entity>,
    boxes: &Vec<DoubleWideBox>,
    coordinate: &GPSCoordinate,
    mov: &Move,
) -> Option<GPSCoordinate> {
    println!("Attempting doublewide move: {:?} {:?}", coordinate, mov);
    if !can_move(map, boxes, coordinate, mov) {
        return None;
    }

    let new_coordinate = coordinate.after_move(mov);

    let this_entity = *map.get(coordinate).unwrap();
    match this_entity {
        Part2Entity::Wall => panic!("Attempted to move a wall entity"),
        Part2Entity::Robot => {
            assert!(try_doublewide_move(map, boxes, &new_coordinate, mov).is_some());
            assert!(map.remove(coordinate).is_some());
            assert!(map.insert(new_coordinate, this_entity).is_none())
        }
        Part2Entity::DoubleWideBox(box_index) => {
            let DoubleWideBox { left, right } = boxes.get(box_index).unwrap().clone();
            let left_new = left.after_move(mov);
            let right_new = right.after_move(mov);

            match (
                try_doublewide_move(map, boxes, &left_new, mov),
                try_doublewide_move(map, boxes, &right_new, mov),
            ) {
                (Some(_), Some(_)) => {
                    assert!(map.remove(&left).is_some());
                    assert!(map.remove(&right).is_some());
                    assert!(map.insert(left_new, this_entity).is_none());
                    assert!(map.insert(right_new, this_entity).is_none());
                }
                (_, _) => panic!("Box push(es) failed"),
            }
        }
    }
    Some(new_coordinate)
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

    // assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(2028, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (singlewide_map, singlewide_robot_coordinate, moves) = parse_input(reader);
        let mut boxes = Vec::new();
        let mut map: HashMap<GPSCoordinate, Part2Entity, RandomState> =
            HashMap::from_iter(singlewide_map.iter().flat_map(|(base_coord, entity)| {
                let (base_x, y) = base_coord.to_xy();
                let left = GPSCoordinate::from_xy(base_x * 2, y);
                let right = GPSCoordinate::from_xy(base_x * 2 + 1, y);

                match entity {
                    Entity::Wall => vec![(left, Part2Entity::Wall), (right, Part2Entity::Wall)],
                    Entity::Box => {
                        let box_index = boxes.len();
                        boxes.push(DoubleWideBox { left, right });
                        vec![
                            (left, Part2Entity::DoubleWideBox(box_index)),
                            (right, Part2Entity::DoubleWideBox(box_index)),
                        ]
                    }
                    Entity::Robot => vec![(left, Part2Entity::Robot)],
                }
            }));
        let singlewide_robot_xy = singlewide_robot_coordinate.to_xy();
        let mut robot_coordinate =
            GPSCoordinate::from_xy(singlewide_robot_xy.0 * 2, singlewide_robot_xy.1);

        for mov in moves {
            match try_doublewide_move(&mut map, &boxes, &robot_coordinate, &mov) {
                Some(new_coord) => robot_coordinate = new_coord,
                None => {}
            }
        }
        Ok(boxes
            .iter()
            .map(|DoubleWideBox { left, right: _ }| left.value)
            .sum())
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
