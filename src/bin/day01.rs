
fn main() {
    let input = include_str!("input/day01_1.txt");
    println!("{}", captcha(input));

    let input2 = include_str!("input/day01_2.txt");
    println!("{}", captcha2(input2));
}


/// The captcha requires you to review a sequence of digits (your puzzle input) 
/// and find the sum of all digits that match the next digit in the list. 
/// The list is circular, so the digit after the last digit is the first digit in the list.
///
/// For example:
///
/// * `1122` produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
/// * `1111` produces 4 because each digit (all 1) matches the next.
/// * `1234` produces 0 because no digit matches the next.
/// * `91212129` produces 9 because the only digit that matches the next one is the last digit, 9.
pub fn captcha(c: &str) -> u32 {
    sum_pairs(c, 1)
}

/// Now, instead of considering the next digit, it wants you to consider the digit 
/// halfway around the circular list. That is, if your list contains 10 items, 
/// only include a digit in your sum if the digit 10/2 = 5 steps forward matches it. 
/// Fortunately, your list has an even number of elements.
/// 
/// For example:
/// 
/// * `1212` produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
/// * `1221` produces 0, because every comparison is between a 1 and a 2.
/// * `123425` produces 4, because both 2s match each other, but no other digit has a match.
/// * `123123` produces 12.
/// * `12131415` produces 4.
pub fn captcha2(c: &str) -> u32 {
    sum_pairs(c, c.len() / 2)
}

/// sums equal pairs of numbers stride items apart
fn sum_pairs(s: &str, stride: usize) -> u32 {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();

    let mut sum = 0;
    for i in 0..len {
        sum += match_val(chars[i], chars[(i + stride) % len]);
    }

    sum
}

fn match_val(x: char, y: char) -> u32 {
    if x == y {
        x.to_digit(10).unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(captcha("1122"), 3);
        assert_eq!(captcha("1111"), 4);
        assert_eq!(captcha("1234"), 0);
        assert_eq!(captcha("91212129"), 9);
    }

    #[test]
    fn sample2() {
        assert_eq!(captcha2("1212"), 6);
        assert_eq!(captcha2("1221"), 0);
        assert_eq!(captcha2("123425"), 4);
        assert_eq!(captcha2("123123"), 12);
        assert_eq!(captcha2("12131415"), 4);
    }
}


// Alternate implementations:

// fn captcha(c: &str) -> u32 {
//     let chars = c.chars().collect::<Vec<_>>();

//     let first_last = match_val(chars[0], chars[chars.len - 1]);

//     chars.windows(2).map(|w| match_val(w[0], w[1])).sum::<u32>() + first_last
// }