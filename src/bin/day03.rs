#![feature(generators)]
#![feature(conservative_impl_trait)]

extern crate gen_iter;
use gen_iter::GenIter;

fn main() {
    let input = 347991;
    println!("{}", steps(input));

    let input2 = 347991;
    println!("{}", part2(input2));
}

/// You come across an experimental new kind of memory stored on an infinite
/// two-dimensional grid.
///
/// Each square on the grid is allocated in a spiral pattern starting at a location
/// marked 1 and then counting up while spiraling outward. For example,
/// the first few squares are allocated like this:
///
/// ```
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23---> ...
/// ```
///
/// While this is very space-efficient (no squares are skipped),
/// requested data must be carried back to square 1
/// (the location of the only access port for this memory system) by programs
/// that can only move up, down, left, or right. They always take the shortest path:
/// the Manhattan Distance between the location of the data and square 1.
///
/// For example:square
///
/// * Data from square 1 is carried 0 steps, since it's at the access port.
/// * Data from square 12 is carried 3 steps, such as: down, left, left.
/// * Data from square 23 is carried only 2 steps: up twice.
/// * Data from square 1024 must be carried 31 steps.
/// How many steps are required to carry the data from the square identified in
/// your puzzle input all the way to the access port?
fn steps(value: u32) -> u32 {
    let location = Grid::find_enumerate(value);

    location.distance()
}

struct Grid {}

#[derive(Debug)]
struct Location(u32, u32);

impl Grid {
    const NEIGHBORS: &'static [(i32, i32)] = &[
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    fn walk() -> impl Iterator<Item = Location> {
        GenIter(|| {
            yield Location(0, 0);

            let mut layer = 1;
            loop {
                for rot in 0..8 * layer {
                    yield Location(layer, rot);
                }
                layer += 1;
            }
        })
    }

    fn find_enumerate(value: u32) -> Location {
        let mut val = 1;
        for loc in Self::walk() {
            if val == value {
                return loc;
            }
            val += 1;
        }

        unreachable!()
    }

    fn spiral_sum() -> impl Iterator<Item = (Location, u32)> {
        let mut cache = [[0; 24]; 24];
        Self::walk().map(move |loc| {
            let (x, y) = loc.as_xy();
            let mut sum = 0;
            for &(dx, dy) in Self::NEIGHBORS {
                sum += cache[(x + dx + 11) as usize][(y + dy + 11) as usize];
            }

            sum = if sum == 0 { 1 } else { sum };

            cache[(x + 11) as usize][(y + 11) as usize] = sum;

            (loc, sum)
        })
    }
}

impl Location {
    fn distance(&self) -> u32 {
        let &Location(layer, _rot) = self;

        if layer == 0 {
            return 0;
        }

        let (x, y) = self.as_xy();

        (x.abs() + y.abs()) as u32
    }

    fn as_xy(&self) -> (i32, i32) {
        let &Location(layer, rot) = self;

        if layer == 0 {
            return (0, 0);
        }

        let side_len = layer * 2;
        let from_corner = (rot + 1) % side_len;

        let half_side = side_len / 2;
        let off_center = from_corner as i32 - half_side as i32;

        let side = ((rot + 1) / side_len) % 4;
        let layer = layer as i32;

        match side {
            0 => (layer, off_center),
            1 => (-off_center, layer),
            2 => (-layer, -off_center),
            3 => (off_center, -layer),
            _ => unreachable!(),
        }
    }
}

/// As a stress test on the system, the programs here clear the grid and then store
/// the value 1 in square 1. Then, in the same allocation order as shown above,
/// they store the sum of the values in all adjacent squares, including diagonals.
///
/// So, the first few squares' values are chosen as follows:
///
/// * Square 1 starts with the value 1.
/// * Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
/// * Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
/// * Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
/// * Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
/// Once a square is written, its value does not change.
/// Therefore, the first few squares would receive the following values:
///
/// ```
/// 147  142  133  122   59
/// 304    5    4    2   57
/// 330   10    1    1   54
/// 351   11   23   25   26
/// 362  747  806--->   ...
/// ```
///
/// What is the first value written that is larger than your puzzle input?
fn part2(value: u32) -> u32 {
    let mut iter = Grid::spiral_sum();
    iter.find(|v| v.1 > value).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(steps(1), 0);
        assert_eq!(steps(12), 3);
        assert_eq!(steps(23), 2);
        assert_eq!(steps(1024), 31);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(10), 11);
        assert_eq!(part2(24), 25);
        assert_eq!(part2(55), 57);
        assert_eq!(part2(800), 806);
    }
}
