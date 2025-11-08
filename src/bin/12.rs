use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::usize;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

// #[derive(Debug)]
struct Region {
    plant: char,
    members: HashSet<Coord>,
    perimeter: usize,
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Region")
            .field("plant", &self.plant)
            .field("area", &self.members.len())
            .field("perimeter", &self.perimeter)
            .finish()
    }
}

impl<T> Index<&Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self[index.y][index.x]
    }
}

fn get_neighbors(Coord { x, y }: &Coord, rows: usize, cols: usize) -> Vec<Coord> {
    let mut neighbors = Vec::<Coord>::new();
    if *x != 0 {
        neighbors.push(Coord { x: x - 1, y: *y });
    }
    if *x != cols - 1 {
        neighbors.push(Coord { x: x + 1, y: *y });
    }
    if *y != 0 {
        neighbors.push(Coord { x: *x, y: y - 1 });
    }
    if *y != rows - 1 {
        neighbors.push(Coord { x: *x, y: y + 1 });
    }
    neighbors
}

fn _show_region(region: &Region) {
    let members = region.members.iter().collect_vec();
    let (x_offset, x_width) = match members.iter().map(|Coord { x, y: _ }| x).minmax() {
        itertools::MinMaxResult::NoElements => (0, 0),
        itertools::MinMaxResult::OneElement(x) => (*x, 1),
        itertools::MinMaxResult::MinMax(min, max) => (*min, 1 + max - min),
    };
    let (y_offset, y_height) = match members.iter().map(|Coord { x: _, y }| y).minmax() {
        itertools::MinMaxResult::NoElements => (0, 0),
        itertools::MinMaxResult::OneElement(y) => (*y, 1),
        itertools::MinMaxResult::MinMax(min, max) => (*min, 1 + max - min),
    };
    let mut map = (0..y_height)
        .map(|_| (0..x_width).map(|_| '.').collect_vec())
        .collect_vec();
    // println!(
    //     "offset,span: x: {},{} y: {},{}",
    //     x_offset, x_width, y_offset, y_height
    // );
    // println!("{:?}", map);
    for Coord { x, y } in members {
        map[y - y_offset][x - x_offset] = region.plant;
    }
    println!(
        "{}",
        map.iter().map(|line| line.into_iter().join("")).join("\n")
    );
}

fn add_region_member_with_bookkeeping(region: &mut Region, member: Coord) {
    if region.members.contains(&member) {
        return;
    }

    let neighbors_in_region = get_neighbors(&member, usize::MAX, usize::MAX)
        .iter()
        .filter(|neighbor| region.members.contains(&neighbor))
        .count();

    match neighbors_in_region {
        0 => {
            // Adding a member in isolation
            region.perimeter = 4;
        }
        1 => {
            // Extending from a point
            region.perimeter += 3 - 1; // Replacing one edge with three
        }
        2 => {
            // Filling in an inset corner
            region.perimeter += 2 - 2; // Replacing two edges with two
        }
        3 => {
            // Filling in an edge inset
            region.perimeter -= 3 - 1; // Replacing three edges with one
        }
        4 => {
            // Filling in an inner gap
            region.perimeter -= 4 - 0; // Replacing four edges with zero
        }
        5.. => panic!(),
    }

    region.members.insert(member);

    // _show_region(region);
    // println!("Perimeter: {}", region.perimeter);
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut plot_coords = Vec::<Coord>::new();

        let map = reader
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, char)| {
                        plot_coords.push(Coord { x, y });
                        char
                    })
                    .collect_vec()
            })
            .collect_vec();
        // println!("{:?}", map);
        let rows = map.len();
        let cols = map[0].len();

        let mut regions = Vec::<Region>::new();

        // Take 1:
        // Iterate every plot coord.
        for coord in plot_coords {
            // println!("\nChecking {:?}", coord);
            let plant = map[&coord];
            let neighbors = get_neighbors(&coord, rows, cols);

            // Look through the coords of the plot's neighbors
            let mut first_found_region: Option<usize> = None;
            for neighbor_coord in neighbors {
                let neighbor_region_idx = regions.iter().position(|region| {
                    region.plant == plant && region.members.contains(&neighbor_coord)
                });
                // println!(
                //     "Neighbor {:?}: region {:?}",
                //     neighbor_coord, neighbor_region_idx
                // );
                match (first_found_region, neighbor_region_idx) {
                    (None, Some(neighbor_region_idx)) => {
                        // If any neighbor is the same type of plant and is in a region, join that region.
                        first_found_region = Some(neighbor_region_idx);
                        add_region_member_with_bookkeeping(
                            &mut regions[neighbor_region_idx],
                            coord.clone(),
                        );
                    }
                    (Some(first_region_idx), Some(neighbor_region_idx)) => {
                        add_region_member_with_bookkeeping(
                            &mut regions[first_region_idx],
                            coord.clone(),
                        );

                        if first_region_idx != neighbor_region_idx {
                            // println!("Coalescing regions");

                            let smaller_region_idx;
                            let larger_region_idx;
                            if regions[first_region_idx].members.len()
                                <= regions[neighbor_region_idx].members.len()
                            {
                                smaller_region_idx = first_region_idx;
                                larger_region_idx = neighbor_region_idx;
                            } else {
                                smaller_region_idx = neighbor_region_idx;
                                larger_region_idx = first_region_idx;
                            }

                            loop {
                                // Find a coord in the neighbor region that is adjacent to something in the current first region and add it
                                let region_members_with_neighbors = regions[larger_region_idx]
                                    .members
                                    .iter()
                                    .filter_map(|coord| {
                                        let neighbors = get_neighbors(coord, rows, cols)
                                            .iter()
                                            .filter(|neighbor_coord| {
                                                regions[smaller_region_idx]
                                                    .members
                                                    .contains(&neighbor_coord)
                                            })
                                            .cloned()
                                            .collect_vec();
                                        match neighbors.is_empty() {
                                            true => None,
                                            false => Some((coord.clone(), neighbors)),
                                        }
                                    })
                                    .collect_vec();

                                for (_, neighbors_in_other_region) in region_members_with_neighbors
                                {
                                    for neighbor in neighbors_in_other_region {
                                        add_region_member_with_bookkeeping(
                                            &mut regions[larger_region_idx],
                                            neighbor.clone(),
                                        );
                                        regions[smaller_region_idx].members.remove(&neighbor);
                                    }
                                }

                                // Loop until neighbor region is empty
                                if regions[smaller_region_idx].members.is_empty() {
                                    break;
                                }
                            }

                            regions.remove(smaller_region_idx);
                        }
                    }
                    (_, None) => {} // Neighbor was not found in a region. Move on, it will end up in a region eventually
                }
            }
            match first_found_region {
                None => {
                    // Create region
                    let mut members = HashSet::new();
                    members.insert(coord);
                    regions.push(Region {
                        plant,
                        members,
                        perimeter: 4,
                    });
                }
                Some(_) => {}
            }
        }

        // println!("{:?}", regions);

        Ok(regions
            .iter()
            .map(|region| region.perimeter * region.members.len())
            .sum())
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

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
