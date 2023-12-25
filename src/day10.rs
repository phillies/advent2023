use std::{
    collections::{BTreeMap, HashMap},
    ops::{Add, Sub},
};

type Tiles = Vec<Vec<char>>;
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

const NORTH: Position = Position { x: 0, y: -1 };
const EAST: Position = Position { x: 1, y: 0 };
const SOUTH: Position = Position { x: 0, y: 1 };
const WEST: Position = Position { x: -1, y: 0 };

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Grid {
    tiles: Tiles,
    start: Position,
}

impl Grid {
    fn get(self: &Grid, position: &Position) -> char {
        if position.y >= 0
            && position.x >= 0
            && position.y < self.tiles.len() as i64
            && position.x < self.tiles[0].len() as i64
        {
            self.tiles[position.y as usize][position.x as usize]
        } else {
            // We return an empty field when we run out of bounds. Simplifies the handling.
            '.'
        }
    }
}

fn find_start(tiles: &Tiles) -> Position {
    for (row_index, row) in tiles.iter().enumerate() {
        for (col_index, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Position {
                    y: row_index as i64,
                    x: col_index as i64,
                };
            }
        }
    }
    Position { x: 0, y: 0 }
}

fn find_first_direction(grid: &Grid, start: &Position) -> Position {
    let north_tile = grid.get(&(start + &NORTH));
    if north_tile == '|' || north_tile == 'F' || north_tile == '7' {
        return NORTH;
    }
    let east_tile = grid.get(&(start + &EAST));
    if east_tile == '-' || east_tile == '7' || east_tile == 'J' {
        return EAST;
    }
    let south_tile = grid.get(&(start + &SOUTH));
    if south_tile == '|' || south_tile == 'J' || south_tile == 'L' {
        return SOUTH;
    }
    let west_tile = grid.get(&(start + &WEST));
    if west_tile == '-' || west_tile == 'F' || west_tile == 'L' {
        return WEST;
    }
    panic!("No direction found!");
}

fn find_path(grid: &Grid, start: &Position) -> Vec<Position> {
    // Read the map like this:
    // If we arrive on tile F and we were walking towards NORTH, next direction is EAST
    let walking_map: HashMap<(char, Position), Position> = [
        (('F', NORTH), EAST),
        (('F', WEST), SOUTH),
        (('J', SOUTH), WEST),
        (('J', EAST), NORTH),
        (('L', SOUTH), EAST),
        (('L', WEST), NORTH),
        (('7', NORTH), WEST),
        (('7', EAST), SOUTH),
        (('|', NORTH), NORTH),
        (('|', SOUTH), SOUTH),
        (('-', WEST), WEST),
        (('-', EAST), EAST),
    ]
    .iter()
    .cloned()
    .collect();

    let mut path = vec![];

    let mut walking_direction = find_first_direction(grid, start);
    let mut next_position = start + &walking_direction;
    while next_position != *start {
        path.push(next_position.clone());
        let tile = grid.get(&next_position);
        walking_direction = walking_map.get(&(tile, walking_direction)).unwrap().clone();
        next_position = next_position + walking_direction;
    }

    path
}

fn generate_grid(input: &Vec<String>) -> Grid {
    let tiles = input
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = find_start(&tiles);

    Grid { tiles, start }
}

fn create_sorted_map(path: &Vec<Position>, grid: &Grid) -> BTreeMap<i64, BTreeMap<i64, char>> {
    let mut positions: BTreeMap<i64, BTreeMap<i64, char>> = BTreeMap::new();
    path.iter().for_each(|t| {
        if !positions.contains_key(&t.y) {
            positions.insert(t.y, BTreeMap::new());
        }
        positions.get_mut(&t.y).unwrap().insert(t.x, grid.get(t));
    });
    positions
}

fn is_switching_side(last_corner: char, this_corner: char) -> bool {
    (last_corner == 'F' && this_corner == 'J') || (last_corner == 'L' && this_corner == '7')
}

fn get_start_tile_type(grid: &Grid, path: &Vec<Position>) -> char {
    let last = path.last().unwrap();
    let first = path.first().unwrap();
    let difference = last - first;

    if difference.x == 0 && difference.y.abs() == 2 {
        '|'
    } else if difference.x.abs() == 2 && difference.y == 0 {
        '-'
    } else if difference.x == difference.y && (last.x > grid.start.x || first.x > grid.start.x) {
        'L'
    } else if difference.x == difference.y && (last.y > grid.start.y || first.y > grid.start.y) {
        '7'
    } else if last.y < grid.start.y || first.y < grid.start.y {
        'J'
    } else {
        'F'
    }
}

fn find_inner_tiles2(positions: &BTreeMap<i64, BTreeMap<i64, char>>, grid: &Grid) -> i64 {
    let mut counter = 0;

    for (row_number, row) in grid.tiles.iter().enumerate() {
        let mut clean_row = vec!['.'; row.len()];
        if !positions.contains_key(&(row_number as i64)) {
            continue;
        }

        let row_tiles = positions.get(&(row_number as i64)).unwrap();
        for (p, c) in row_tiles.iter() {
            clean_row.insert(*p as usize, *c);
        }
        let mut is_inside = false;
        let mut last_corner = '.';

        for c in clean_row.iter() {
            match *c {
                '|' => is_inside = !is_inside,
                '.' => {
                    if is_inside {
                        counter += 1;
                    }
                }
                'F' => {
                    last_corner = *c;
                }
                'L' => {
                    last_corner = *c;
                }
                '7' => {
                    if is_switching_side(last_corner, *c) {
                        is_inside = !is_inside;
                    }
                    last_corner = '.';
                }
                'J' => {
                    if is_switching_side(last_corner, *c) {
                        is_inside = !is_inside;
                    }
                    last_corner = '.';
                }
                _ => {}
            }
        }
    }

    counter
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let grid = generate_grid(&input);

    let path = find_path(&grid, &grid.start);

    let max_distance = (path.len() as f64 / 2.0).ceil() as i64;

    let mut positions = create_sorted_map(&path, &grid);

    let start_tile = get_start_tile_type(&grid, &path);
    positions
        .get_mut(&grid.start.y)
        .unwrap()
        .insert(grid.start.x, start_tile);

    let inner_tiles = find_inner_tiles2(&positions, &grid);

    (max_distance, inner_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day10() {
        let result_1 = 8;
        let result_2 = 1;
        let input = vec![
            "|.F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ.JF".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day10_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day10.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
