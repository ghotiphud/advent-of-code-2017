/// Hex grid info
/// https://www.redblobgames.com/grids/hexagons/
fn main() {
    let input = include_str!("input/day11.txt");
    println!("{}", part1(input));
}

/// The hexagons ("hexes") in this grid are aligned such that adjacent hexes can
/// be found to the north, northeast, southeast, south, southwest, and northwest:
///
///   \ n  /
/// nw +--+ ne
///   /    \
/// -+      +-
///   \    /
/// sw +--+ se
///   / s  \
/// You have the path the child process took. Starting where he started, you
/// need to determine the fewest number of steps required to reach him. (A
/// "step" means to move from the hex you are in to any adjacent hex.)
///
/// For example:
///
/// ne,ne,ne is 3 steps away.
/// ne,ne,sw,sw is 0 steps away (back where you started).
/// ne,ne,s,s is 2 steps away (se,se).
/// se,sw,se,sw,sw is 3 steps away (s,s,sw).
///
/// --- Part Two ---
///
/// How many steps away is the furthest he ever got from his starting position?
fn part1(s: &str) -> i32 {
    let origin = HexCoord { q: 0, r: 0 };
    let mut pos = origin.clone();
    let mut farthest = 0;
    for dir in s.split(",") {
        pos = pos.step(dir);

        let dist = pos.distance_to(&origin);
        if dist > farthest {
            farthest = dist;
        }
    }

    println!("{}", farthest);
    pos.distance_to(&origin)
}

/// q: column, r: row
#[derive(Debug, Clone)]
struct HexCoord {
    q: i32,
    r: i32,
}

impl HexCoord {
    fn step(&self, dir: &str) -> HexCoord {
        let (dq, dr) = match dir {
            "n" => (0, -1),
            "ne" => (1, -1),
            "se" => (1, 0),
            "s" => (0, 1),
            "sw" => (-1, 1),
            "nw" => (-1, 0),
            _ => unreachable!("unexpected"),
        };

        HexCoord {
            q: self.q + dq,
            r: self.r + dr,
        }
    }

    fn distance_to(&self, c2: &HexCoord) -> i32 {
        let (x1, y1, z1) = (self.q, self.r, -self.q - self.r);
        let (x2, y2, z2) = (c2.q, c2.r, -c2.q - c2.r);

        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }
}
