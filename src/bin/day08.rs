use std::str;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate nom;
use nom::{alpha, digit, multispace, space};

fn main() {
    let input = include_str!("input/day08.txt");
    println!("{}", part1(input));

    println!("{}", part2(input));
}

struct CPU<'a> {
    registers: HashMap<&'a str, i32>,
    max_ever: i32,
}

impl<'a> CPU<'a> {
    fn compute(&mut self, prog: Vec<Instruction<'a>>) {
        for inst in prog {
            self.run_instruction(inst);
        }
    }

    fn max_register(&self) -> i32 {
        println!("{:?}", self.registers);
        self.registers.values().max().expect("reg vals").clone()
    }

    fn run_instruction(&mut self, inst: Instruction<'a>) {
        let cmp_reg_val = *self.registers.entry(inst.reg_cmp).or_insert(0);

        let cmp_res = match inst.cmp_op {
            ">" => cmp_reg_val > inst.cmp_to,
            "<" => cmp_reg_val < inst.cmp_to,
            ">=" => cmp_reg_val >= inst.cmp_to,
            "<=" => cmp_reg_val <= inst.cmp_to,
            "==" => cmp_reg_val == inst.cmp_to,
            "!=" => cmp_reg_val != inst.cmp_to,
            _ => panic!("unhandled op"),
        };

        if cmp_res {
            let reg_val = self.registers.entry(inst.reg).or_insert(0);

            match inst.action {
                "inc" => *reg_val += inst.amount,
                "dec" => *reg_val -= inst.amount,
                _ => panic!("unhandled reg action"),
            }

            if *reg_val > self.max_ever {
                self.max_ever = *reg_val;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    reg: &'a str,
    action: &'a str,
    amount: i32,

    reg_cmp: &'a str,
    cmp_op: &'a str,
    cmp_to: i32,
}

/// Q1
fn part1(s: &str) -> i32 {
    let parsed = parse_program(s.as_bytes());

    if let nom::IResult::Done(_, instructions) = parsed {
        let mut cpu = CPU {
            registers: HashMap::new(),
            max_ever: 0,
        };

        cpu.compute(instructions);

        cpu.max_register()
    } else {
        panic!("parse error")
    }
}

/// Q2
fn part2(s: &str) -> i32 {
    let parsed = parse_program(s.as_bytes());

    if let nom::IResult::Done(_, instructions) = parsed {
        let mut cpu = CPU {
            registers: HashMap::new(),
            max_ever: 0,
        };

        cpu.compute(instructions);

        cpu.max_ever
    } else {
        panic!("parse error")
    }
}

// Parser
named!(int32(&[u8]) -> i32, 
    map!(
        pair!(
            opt!(tag!("-")),
            map_res!(digit, str::from_utf8)
        ),
        |(s,n)| n.parse::<i32>().expect("digits") * if s.is_some() {-1} else {1}
    )
);

named!(
    parse_instruction(&[u8]) -> Instruction,
    do_parse!(
        reg: map_res!(alpha, str::from_utf8) >>
        space >>
        action: map_res!(alpha, str::from_utf8) >>
        space >>
        amount: int32 >>
        tag!(" if ") >>
        reg_cmp: map_res!(alpha, str::from_utf8) >>
        space >>
        cmp_op: map_res!(is_a_s!("<>!="), str::from_utf8) >>
        space >>
        cmp_to: int32 >>
        opt!(complete!(multispace)) >>

        (Instruction {
            reg: reg,
            action: action,
            amount: amount,
            reg_cmp: reg_cmp,
            cmp_op: cmp_op,
            cmp_to: cmp_to,
        })
    )
);

named!(
    parse_program(&[u8]) -> Vec<Instruction>,
    many0!(parse_instruction)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(part1(input), 1);
    }

    #[test]
    fn parse_test() {
        let input = "b inc 5 if a > 1";

        assert_eq!(
            parse_instruction(input.as_bytes()),
            nom::IResult::Done(
                "".as_bytes(),
                Instruction {
                    reg: "b",
                    action: "inc",
                    amount: 5,
                    reg_cmp: "a",
                    cmp_op: ">",
                    cmp_to: 1,
                }
            )
        );
    }

    #[test]
    fn sample2() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(part2(input), 10);
    }
}
