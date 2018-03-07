fn main() {
    let input = include_str!("input/day09.txt");
    println!("{}", part1(input));
}

/// You sit for a while and record part of the stream (your puzzle input).
/// The characters represent groups - sequences that begin with { and end
/// with }. Within a group, there are zero or more other things, separated
/// by commas: either another group or garbage. Since groups can contain
/// other groups, a } only closes the most-recently-opened unclosed group
/// - that is, they are nestable. Your puzzle input represents a single,
/// large group which itself contains many smaller ones.
///
/// Sometimes, instead of a group, you will find garbage. Garbage begins
/// with < and ends with >. Between those angle brackets, almost any
/// character can appear, including { and }. Within garbage, < has no
/// special meaning.
///
/// In a futile attempt to clean up the garbage, some program has canceled
/// some of the characters within it using !: inside garbage, any character
/// that comes after ! should be ignored, including <, >, and even another !.
///
/// You don't see any characters that deviate from these rules. Outside
/// garbage, you only find well-formed groups, and garbage always
/// terminates according to the rules above.
///
/// Here are some self-contained pieces of garbage:
///
/// <>, empty garbage.
/// <random characters>, garbage containing random characters.
/// <<<<>, because the extra < are ignored.
/// <{!>}>, because the first > is canceled.
/// <!!>, because the second ! is canceled, allowing the > to
/// terminate the garbage.
/// <!!!>>, because the second ! and the first > are canceled.
/// <{o"i!a,<{i<a>, which ends at the first >.
/// Here are some examples of whole streams and the number of groups
/// they contain:
///
/// {}, 1 group.
/// {{{}}}, 3 groups.
/// {{},{}}, also 3 groups.
/// {{{},{},{{}}}}, 6 groups.
/// {<{},{},{{}}>}, 1 group (which itself contains garbage).
/// {<a>,<a>,<a>,<a>}, 1 group.
/// {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
/// {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).
/// Your goal is to find the total score for all groups in your input.
/// Each group is assigned a score which is one more than the score of
/// the group that immediately contains it. (The outermost group gets a
/// score of 1.)
///
/// {}, score of 1.
/// {{{}}}, score of 1 + 2 + 3 = 6.
/// {{},{}}, score of 1 + 2 + 2 = 5.
/// {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
/// {<a>,<a>,<a>,<a>}, score of 1.
/// {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
/// {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
/// {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
/// What is the total score for all groups in your input?
fn part1(s: &str) -> i32 {
    let mut score = 0;
    let mut depth = 0;
    let mut garbage = false;
    let mut garbage_count = 0;
    let mut ignore_next = false;

    for c in s.chars() {
        if c == '!' || ignore_next {
            ignore_next = !ignore_next;
            continue;
        }

        if garbage && c != '>' {
            garbage_count += 1;
            continue;
        }

        match c {
            '{' => {
                depth += 1;
                score += depth;
            } // group
            '}' => depth -= 1,
            '<' => garbage = true,  // garbage
            '>' => garbage = false, // end garbage
            _ => {}
        }
    }

    println!("{}", garbage_count);
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(part1("{}"), 1);
        assert_eq!(part1("{{{}}}"), 6);
        assert_eq!(part1("{{},{}}"), 5);
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
        assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
