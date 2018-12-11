extern crate failure;

use failure::Error;

fn maion() -> Result<(), Error> {
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
