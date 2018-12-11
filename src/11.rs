extern crate failure;

use failure::Error;
use std::collections::HashMap;
use std::i64;

const GRID_SIZE: i64 = 300;

fn main() -> Result<(), Error> {
    let input = 5719;
    let grid = Grid::new(input);
    let (x, y) = grid.best_square();
    println!("Part 1: {},{}", x, y);
    Ok(())
}

#[derive(Debug)]
struct Grid {
    serial_number: i64,
}

impl Grid {
    fn new(serial_number: i64) -> Grid {
        Grid {
            serial_number: serial_number,
        }
    }

    fn power_level(&self, x: i64, y: i64) -> i64 {
        let rack_id = x + 10;
        let mut power_level = rack_id * y;
        power_level += self.serial_number;
        power_level *= rack_id;
        power_level = (power_level / 100) % 10;
        power_level - 5
    }

    fn best_square(&self) -> (i64, i64) {
        let mut power_levels = HashMap::new();
        for y in 1..=GRID_SIZE {
            for x in 1..=GRID_SIZE {
                power_levels.insert((x, y), self.power_level(x, y));
            }
        }
        let mut largest_total_power = i64::MIN;
        let mut best_square = (0, 0);
        for y in 1..(GRID_SIZE - 1) {
            for x in 1..(GRID_SIZE - 1) {
                let mut total_power = 0;
                for y in y..=(y + 2) {
                    for x in x..=(x + 2) {
                        total_power += power_levels.get(&(x, y)).unwrap();
                    }
                }
                if total_power > largest_total_power {
                    largest_total_power = total_power;
                    best_square = (x, y);
                }
            }
        }
        best_square
    }
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
