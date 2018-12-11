extern crate failure;

use failure::Error;
use std::collections::HashMap;
use std::i64;

const GRID_SIZE: i64 = 300;

fn main() {
    let input = 5719;
    let grid = Grid::new(input);
    println!("Part 1: {}", grid.part_1());
    println!("Part 2: {}", grid.part_2());
}

#[derive(Debug)]
struct Grid {
    power_levels: HashMap<(i64, i64), i64>,
}

#[derive(Debug)]
struct Square {
    x: i64,
    y: i64,
    size: i64,
    total_power: i64,
}

impl Grid {
    fn new(serial_number: i64) -> Grid {
        let mut power_levels = HashMap::new();
        for y in 1..=GRID_SIZE {
            for x in 1..=GRID_SIZE {
                power_levels.insert((x, y), power_level(x, y, serial_number));
            }
        }
        Grid {
            power_levels: power_levels,
        }
    }

    fn part_1(&self) -> String {
        let square = self.best_3x3_square();
        format!("{},{}", square.x, square.y)
    }

    fn part_2(&self) -> String {
        let square = self.best_square_of_any_size();
        format!("{},{},{}", square.x, square.y, square.size)
    }

    fn best_3x3_square(&self) -> Square {
        self.best_square(3)
    }

    fn best_square_of_any_size(&self) -> Square {
        unimplemented!()
    }

    fn best_square(&self, size: i64) -> Square {
        unimplemented!()
    }
}

fn power_level(x: i64, y: i64, serial_number: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

#[test]
fn part_1() {
    let grid = Grid::new(8);
    assert_eq!(4, grid.power_level(3, 5));

    let grid = Grid::new(57);
    assert_eq!(-5, grid.power_level(122, 79));

    let grid = Grid::new(39);
    assert_eq!(0, grid.power_level(217, 196));

    let grid = Grid::new(71);
    assert_eq!(4, grid.power_level(101, 153));
}
