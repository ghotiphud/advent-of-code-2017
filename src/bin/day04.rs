#![feature(generators)]
#![feature(conservative_impl_trait)]

extern crate gen_iter;
use gen_iter::GenIter;

extern crate itertools;
use itertools::Itertools;

use std::collections::HashSet;

fn main() {
    let input = include_str!("input/day04_1.txt");
    println!("{}", part1(input));

    println!("{}", part2(input));
}

/// A new system policy has been put in place that requires all accounts to use a
/// passphrase instead of simply a password. A passphrase consists of a series of words
/// (lowercase letters) separated by spaces.
///
/// To ensure security, a valid passphrase must contain no duplicate words.
///
/// For example:
///
/// * aa bb cc dd ee is valid.
/// * aa bb cc dd aa is not valid - the word aa appears more than once.
/// * aa bb cc dd aaa is valid - aa and aaa count as different words.
/// The system's full passphrase list is available as your puzzle input.
/// How many passphrases are valid?
fn part1(s: &str) -> usize {
    s.lines().map(valid_pass).filter(|&valid| valid).count()
}

fn valid_pass(pass: &str) -> bool {
    let mut words = HashSet::new();

    for word in pass.split_whitespace() {
        let inserted = words.insert(word);

        if !inserted {
            return false;
        }
    }

    true
}

/// For added security, yet another system policy has been put in place.
/// Now, a valid passphrase must contain no two words that are anagrams of each other -
/// that is, a passphrase is invalid if any word's letters can be rearranged to form
/// any other word in the passphrase.
///
/// For example:
///
/// * abcde fghij is a valid passphrase.
/// * abcde xyz ecdab is not valid - the letters from the third word can be rearranged
///     to form the first word.
/// * a ab abc abd abf abj is a valid passphrase, because all letters need to be used
///     when forming another word.
/// * iiii oiii ooii oooi oooo is valid.
/// * oiii ioii iioi iiio is not valid - any of these words can be rearranged to form
///     any other word.
/// Under this new system policy, how many passphrases are valid?
fn part2(s: &str) -> usize {
    s.lines().map(valid_pass2).filter(|&valid| valid).count()
}

fn valid_pass2(pass: &str) -> bool {
    let mut words = HashSet::new();

    for word in pass.split_whitespace() {
        let sorted: String = word.chars().sorted().into_iter().collect();

        let inserted = words.insert(sorted);

        if !inserted {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(part1("aa bb cc dd ee"), 1);
        assert_eq!(part1("aa bb cc dd aa"), 0);
        assert_eq!(part1("aa bb cc dd aaa"), 1);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2("abcde fghij"), 1);
        assert_eq!(part2("abcde xyz ecdab"), 0);
        assert_eq!(part2("a ab abc abd abf abj"), 1);
        assert_eq!(part2("iiii oiii ooii oooi oooo"), 1);
        assert_eq!(part2("oiii ioii iioi iiio"), 0);
    }
}
