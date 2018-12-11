use std::collections::HashMap;
use std::i64;

fn main() {
    let input = 5719;
    let size = 300;
    let grid = Grid::new(size, input);
    println!("Part 1: {}", grid.part_1());
    println!("Part 2: {}", grid.part_2());
}

#[derive(Debug)]
struct Grid {
    power_levels: HashMap<(i64, i64), i64>,
    size: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Square {
    total_power: i64,
    size: i64,
    x: i64,
    y: i64,
}

impl Grid {
    fn new(size: i64, serial_number: i64) -> Grid {
        let mut power_levels = HashMap::new();
        for y in 1..=size {
            for x in 1..=size {
                power_levels.insert((x, y), power_level(x, y, serial_number));
            }
        }
        Grid {
            power_levels: power_levels,
            size: size,
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
        let mut best_square = Square::worst();
        for size in 1..=self.size {
            let square = self.best_square(size);
            if square > best_square {
                best_square = square;
            }
        }
        best_square
    }

    fn best_square(&self, size: i64) -> Square {
        let mut best_square = Square::worst();
        for x in 1..=(self.size - size + 1) {
            for y in 1..=(self.size - size + 1) {
                let square = self.square(x, y, size).unwrap();
                if square > best_square {
                    best_square = square;
                }
            }
        }
        best_square
    }

    fn square(&self, x: i64, y: i64, size: i64) -> Option<Square> {
        let mut total_power = 0;
        for x in x..(x + size) {
            for y in y..(y + size) {
                if let Some(power) = self.power_levels.get(&(x, y)) {
                    total_power += power;
                } else {
                    return None;
                }
            }
        }
        Some(Square {
            x: x,
            y: y,
            size: size,
            total_power: total_power,
        })
    }
}

impl Square {
    fn worst() -> Square {
        Square {
            x: 0,
            y: 0,
            size: 0,
            total_power: i64::MIN,
        }
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
