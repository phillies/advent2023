use itertools::Itertools;
use std::{fmt, ops::Sub};

use num::Integer;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Galaxy {
    sectors: Vec<char>,
    width: usize,
    height: usize,
}

struct AncientGalaxy {
    sectors: Vec<char>,
    width: usize,
    height: usize,
    expansion_size: i64,
}

trait SpaceExploration {
    // fn position_to_index(self: &Self, position: &Position) -> i64;
    fn get(self: &Self, x: usize, y: usize) -> char {
        *self
            .get_sectors()
            .get(y * self.get_width() + x)
            .unwrap_or(&'.')
    }
    fn get_height(self: &Self) -> usize;
    fn get_width(self: &Self) -> usize;
    fn get_sectors(self: &Self) -> &Vec<char>;
    fn index_to_position(self: &Self, index: i64) -> Position {
        Position {
            x: index.mod_floor(&(self.get_width() as i64)),
            y: index.div_euclid(self.get_width() as i64),
        }
    }
    fn get_empty_rows(self: &Self) -> Vec<usize> {
        let mut empty_rows = vec![];
        let width = self.get_width();
        let sectors = self.get_sectors();
        for ii in 0..self.get_height() {
            if sectors
                .get(ii * width..(ii + 1) * width)
                .unwrap()
                .iter()
                .all(|c| *c == '.')
            {
                empty_rows.push(ii);
            }
        }
        empty_rows
    }
    fn get_empty_cols(self: &Self) -> Vec<usize> {
        let mut empty_cols = vec![];
        let width = self.get_width();
        let sectors = self.get_sectors();

        for ii in 0..width {
            if sectors.iter().skip(ii).step_by(width).all(|c| *c == '.') {
                empty_cols.push(ii);
            }
        }
        empty_cols
    }
    fn expand(self: &mut Self);

    fn calculate_distance(self: &Self, left: &Position, right: &Position) -> i64;
}

// Blanket implementation for all types that implement SpaceExploration
pub struct DisplaySpace<T>(T);

impl<T: SpaceExploration> fmt::Display for DisplaySpace<&'_ T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ii in 0..self.0.get_height() {
            for jj in 0..self.0.get_width() {
                write!(f, "{}", self.0.get(jj, ii))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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

impl SpaceExploration for Galaxy {
    fn get_sectors(self: &Galaxy) -> &Vec<char> {
        &self.sectors
    }

    fn get_height(self: &Self) -> usize {
        self.height
    }

    fn get_width(self: &Self) -> usize {
        self.width
    }
    fn expand(self: &mut Galaxy) {
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        // Could be more efficient if we first do all the splits and then insert the empty rows
        // but this makes it more complicated to implement, and can be seen as premature optimization
        for row_number in empty_rows.iter().rev() {
            let empty_row = vec!['.'; self.width];
            let split_sectors = self.sectors.split_at(*row_number * self.width);

            self.sectors = [split_sectors.0, &empty_row, split_sectors.1].concat();
        }
        self.height += empty_rows.len();

        for column_number in empty_cols.iter().rev() {
            // Because we are going backwards, we don't need to adjust for the inserted empty fields
            for ii in (0..self.height).rev() {
                self.sectors.insert(column_number + ii * self.width, '.');
            }
            self.width += 1;
        }
    }

    fn calculate_distance(self: &Self, left: &Position, right: &Position) -> i64 {
        let diff = left - right;
        diff.x.abs() + diff.y.abs()
    }
}

// Create a range from a to b, excluding a and a can be larger than b
fn range_exclusive_start(a: i64, b: i64) -> impl Iterator<Item = i64> {
    let x: Box<dyn Iterator<Item = i64>>;
    if b > a {
        x = Box::new((a + 1)..=b)
    } else {
        x = Box::new((b..a).rev())
    }
    x
}

impl SpaceExploration for AncientGalaxy {
    fn expand(self: &mut AncientGalaxy) {
        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        for row_number in empty_rows.iter() {
            for ii in 0..self.width {
                self.sectors[*row_number * self.width + ii] = 'x';
            }
        }

        for column_number in empty_cols.iter() {
            // Because we are going backwards, we don't need to adjust for the inserted empty fields
            for ii in 0..self.height {
                self.sectors[column_number + ii * self.width] = 'x';
            }
        }
    }

    fn calculate_distance(self: &AncientGalaxy, left: &Position, right: &Position) -> i64 {
        let mut distance = 0;
        if left.x != right.x {
            for x in range_exclusive_start(left.x, right.x) {
                match self.get(x as usize, left.y as usize) {
                    'x' => distance += self.expansion_size as i64,
                    _ => distance += 1,
                }
            }
        }
        if left.y != right.y {
            for y in range_exclusive_start(left.y, right.y) {
                match self.get(right.x as usize, y as usize) {
                    'x' => distance += self.expansion_size as i64,
                    _ => distance += 1,
                }
            }
        }
        distance as i64
    }

    fn get_height(self: &Self) -> usize {
        self.height
    }

    fn get_width(self: &Self) -> usize {
        self.width
    }

    fn get_sectors(self: &Self) -> &Vec<char> {
        &self.sectors
    }
}

fn get_stars(galaxy: &dyn SpaceExploration) -> Vec<Position> {
    galaxy
        .get_sectors()
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            if *entry == '#' {
                Some(galaxy.index_to_position(index as i64))
            } else {
                None
            }
        })
        .collect()
}

fn solve_part_1(input: &Vec<String>) -> i64 {
    let mut galaxy = Galaxy {
        sectors: input.iter().flat_map(|s| s.chars()).collect::<Vec<char>>(),
        width: input[0].len(),
        height: input.len(),
    };
    galaxy.expand();

    let stars = get_stars(&galaxy);

    stars
        .iter()
        .combinations(2)
        .into_iter()
        .map(|c| {
            let left = c[0];
            let right = c[1];
            galaxy.calculate_distance(left, right)
        })
        .sum::<i64>()
}

fn solve_part_2(input: &Vec<String>, empty_size: i64) -> i64 {
    let mut ancient_galaxy = AncientGalaxy {
        sectors: input.iter().flat_map(|s| s.chars()).collect::<Vec<char>>(),
        width: input[0].len(),
        height: input.len(),
        expansion_size: empty_size,
    };

    ancient_galaxy.expand();

    let stars = get_stars(&ancient_galaxy);

    let distances = stars
        .iter()
        .combinations(2)
        .into_iter()
        .map(|c| ancient_galaxy.calculate_distance(c[0], c[1]))
        .sum::<i64>();

    distances
}

pub fn solve(input: &Vec<String>) -> (i64, i64) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input, 1000000);

    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day11() {
        let result_1 = 374;
        let result_2 = 82000210;
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        let (output_1, output_2) = solve(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[test]
    fn test_day11_part_1() {
        let result_1 = 374;
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        let output_1 = solve_part_1(&input);
        assert_eq!(result_1, output_1);
    }

    #[test]
    fn test_day11_part_2() {
        let result_1 = 1030;
        let input = vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ];
        let output_1 = solve_part_2(&input, 10);
        assert_eq!(result_1, output_1);
    }

    #[bench]
    fn bench_day11_part_1(b: &mut Bencher) {
        let input = read_input_to_vector("data/day11.txt");
        b.iter(|| {
            solve(&input);
        });
    }
}
