// #![feature(generators)]
// #![feature(conservative_impl_trait)]

// use std::collections::{ HashMap, HashSet };

// extern crate gen_iter;
// use gen_iter::GenIter;

// extern crate itertools;
// use itertools::Itertools;

fn main() {
    let input = "157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30";
    println!("{}", part1((0..256).collect(), input));

    println!("{}", part2((0..256).collect(), input));
}

/// To achieve this, begin with a list of numbers from 0 to 255, a current
/// position which begins at 0 (the first element in the list), a skip size
/// (which starts at 0), and a sequence of lengths (your puzzle input). Then,
/// for each length:
///
/// * Reverse the order of that length of elements in the list, starting with the
///   element at the current position.
/// * Move the current position forward by that length plus the skip size.
/// * Increase the skip size by one.
///
/// The list is circular; if the current position and the length try to reverse
/// elements beyond the end of the list, the operation reverses using as many
/// extra elements as it needs from the front of the list. If the current
/// position moves past the end of the list, it wraps around to the front.
/// Lengths larger than the size of the list are invalid.
///
/// Here's an example using a smaller list:
///
/// Suppose we instead only had a circular list containing five elements,
/// 0, 1, 2, 3, 4, and were given input lengths of 3, 4, 1, 5.
///
/// The list begins as [0] 1 2 3 4 (where square brackets indicate the current
/// position).
/// The first length, 3, selects ([0] 1 2) 3 4 (where parentheses indicate the
/// sublist to be reversed).
/// After reversing that section (0 1 2 into 2 1 0), we get ([2] 1 0) 3 4.
/// Then, the current position moves forward by the length, 3, plus the skip
/// size, 0: 2 1 0 [3] 4. Finally, the skip size increases to 1.
/// The second length, 4, selects a section which wraps: 2 1) 0 ([3] 4.
/// The sublist 3 4 2 1 is reversed to form 1 2 4 3: 4 3) 0 ([1] 2.
/// The current position moves forward by the length plus the skip size, a total
/// of 5, causing it not to move because it wraps around: 4 3 0 [1] 2. The skip
/// size increases to 2.
/// The third length, 1, selects a sublist of a single element, and so reversing
/// it has no effect.
/// The current position moves forward by the length (1) plus the skip size
/// (2): 4 [3] 0 1 2. The skip size increases to 3.
/// The fourth length, 5, selects every element starting with the second: 4)
/// ([3] 0 1 2. Reversing this sublist (3 0 1 2 4 into 4 2 1 0 3) produces:
/// 3) ([4] 2 1 0.
/// Finally, the current position moves forward by 8: 3 4 2 1 [0]. The skip size
/// increases to 4.
/// In this example, the first two numbers in the list end up being 3 and 4; to
/// check the process, you can multiply them together to produce 12.
///
/// However, you should instead use the standard list size of 256 (with values 0
/// to 255) and the sequence of lengths in your puzzle input. Once this process
/// is complete, what is the result of multiplying the first two numbers in the
/// list?
fn part1(list: Vec<i32>, s: &str) -> i32 {
    let mut list = list;

    let lengths: Vec<usize> = s.split(",")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    hash(&mut list, &lengths, 0, 0);

    list[0] * list[1]
}

/// input -> ascii codes + 17, 31, 73, 47, 23
/// dense hash -> chunk(16), fold with XOR, then print as hex
fn part2(list: Vec<i32>, s: &str) -> String {
    let mut list = list;
    let mut pos = 0;
    let mut skip = 0;

    let lengths: Vec<usize> = s.as_bytes()
        .iter()
        .map(|&b| b as usize)
        .chain(vec![17, 31, 73, 47, 23])
        .collect();

    for _round in 0..64 {
        let (p, s) = hash(&mut list, &lengths, pos, skip);
        pos = p;
        skip = s;
    }

    to_dense_hash(&list)
}

fn hash(list: &mut Vec<i32>, lengths: &[usize], mut pos: usize, mut skip: usize) -> (usize, usize) {
    let list_len = list.len();

    for len in lengths {
        let end = pos + len;

        let indx_iter = (pos..end).map(|i| i % list_len);
        let indx_iter2 = (pos..end).map(|i| i % list_len);
        let tmp_list = list.clone();

        for (i, j) in indx_iter.zip(indx_iter2.rev()) {
            list[j] = tmp_list[i];
        }

        pos = (pos + len + skip) % list_len;
        skip += 1;
    }

    (pos, skip)
}

// Alt reversing
// match (pos, end) {
//     (s, e) if e < list_len => {
//         list[s..e].reverse();
//     }
//     (s, e) if e > list_len => {
//         let replace: Vec<_> = list[s..]
//             .iter()
//             .cloned()
//             .chain(list[0..e % list_len].iter().cloned())
//             .rev()
//             .collect();

//         list.splice(s..list_len, replace[0..list_len - s].iter().cloned());
//         list.splice(0..e % list_len, replace[list_len - s..].iter().cloned());
//     }
// }

fn to_dense_hash(list: &[i32]) -> String {
    let mut dense = String::new();

    for block in list.chunks(16) {
        dense += &format!("{:02x}", block.iter().fold(0, |acc, &num| acc ^ num));
    }

    dense
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(part1((0..5).collect(), "3,4,1,5"), 12);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2((0..256).collect(), ""),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            part2((0..256).collect(), "AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            part2((0..256).collect(), "1,2,3"),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            part2((0..256).collect(), "1,2,4"),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
