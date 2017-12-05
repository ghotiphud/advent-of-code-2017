extern crate itertools;
use itertools::{Itertools, MinMaxResult};

fn main() {
    let input = include_str!("input/day02.txt");
    println!("{}", checksum(input));

    println!("{}", divisible_sum(input));
}

/// The spreadsheet consists of rows of apparently-random numbers.
/// To make sure the recovery process is on the right track,
/// they need you to calculate the spreadsheet's checksum. For each row,
/// determine the difference between the largest value and the smallest value;
/// the checksum is the sum of all of these differences.
///
/// For example, given the following spreadsheet:
///
/// ```
/// 5 1 9 5
/// 7 5 3
/// 2 4 6 8
/// ```
///
/// * The first row's largest and smallest values are 9 and 1, and their difference is 8.
/// * The second row's largest and smallest values are 7 and 3, and their difference is 4.
/// * The third row's difference is 6.
/// In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
fn checksum(s: &str) -> u32 {
    s.lines()
        .map(parse_line)
        .map(line_checksum)
        .sum()
}

fn line_checksum(l: Vec<u32>) -> u32 {
    l.iter().max().unwrap() - l.iter().min().unwrap()
}

fn line_checksum2(l: Vec<u32>) -> u32 {
    match l.iter().minmax() {
        MinMaxResult::MinMax(x,y) => y-x,
        _ => panic!()
    }
}

/// It sounds like the goal is to find the only two numbers in each row where one
/// evenly divides the other - that is, where the result of the division operation
/// is a whole number. They would like you to find those numbers on each line,
/// divide them, and add up each line's result.
///
/// For example, given the following spreadsheet:
///
/// ```
/// 5 9 2 8
/// 9 4 7 3
/// 3 8 6 5
/// ```
///
/// * In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
/// * In the second row, the two numbers are 9 and 3; the result is 3.
/// * In the third row, the result is 2.
/// In this example, the sum of the results would be 4 + 3 + 2 = 9.
fn divisible_sum(s: &str) -> u32 {
    s.lines()
        .map(parse_line)
        .map(line_divisible)
        .sum()
}

fn line_divisible(line: Vec<u32>) -> u32 {
    for (x, y) in line.iter().tuple_combinations() {
        if x % y == 0 {
            return x / y;
        } else if y % x == 0 {
            return y / x;
        }
    }

    0
}

// Helpers
fn parse_line(l: &str) -> Vec<u32> {
    l.split_whitespace().map(|v| v.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";

        assert_eq!(checksum(input), 18);
    }

    #[test]
    fn sample2() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";

        assert_eq!(divisible_sum(input), 9);
    }
}
