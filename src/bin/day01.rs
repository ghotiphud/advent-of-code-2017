fn main() {
    let input = include_str!("day01_1.txt");

    println!("{}", captcha(input));

    let input2 = include_str!("day01_2.txt");

    println!("{}", captcha2(input2));
}

fn captcha(c: &str) -> u32 {
    sum_pairs(c, 1)

    // let chars = c.chars().collect::<Vec<_>>();

    // let first_last = match_val(chars[0], chars.last().unwrap().clone());

    // chars.windows(2).map(|w| match_val(w[0], w[1])).sum::<u32>() + first_last
}

fn captcha2(c: &str) -> u32 {
    sum_pairs(c, c.len() / 2)
}

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
