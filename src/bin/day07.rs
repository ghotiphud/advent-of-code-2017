use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate nom;
use nom::{alpha, multispace, space};
use std::str;

fn main() {
    let input = include_str!("input/day07.txt");
    println!("{}", part1(input));

    println!("{}", part2(input));
}

/// Q1
fn part1(s: &str) -> String {
    let tower = to_tower(s);

    tower.find_root().to_owned()
}

#[derive(Debug)]
struct Tower<'a> {
    data: HashMap<&'a str, TowerNode<'a>>,
}

#[derive(Debug)]
struct TowerNode<'a> {
    weight: i32,
    supporting: Vec<&'a str>,
    sup_weight: i32,
    parent: Option<&'a str>,
}

impl<'t> TowerNode<'t> {
    fn imbalance(&self, tower: &Tower<'t>) -> Option<(&str, i32)> {
        let mut weights = Vec::new();
        let mut weight_freq = HashMap::new();
        for s in &self.supporting {
            let supported = &tower.data[s];
            let bal_weight = supported.sup_weight;
            *weight_freq.entry(bal_weight).or_insert(0) += 1;
            weights.push((s.clone(), bal_weight));
        }

        // println!("{:?}", weight_freq);
        for (w1, times) in weight_freq {
            if times == 1 {
                let this = *weights.iter().filter(|&&(s, bw)| bw == w1).next().unwrap();
                let other = *weights.iter().filter(|&&(s, bw)| bw != w1).next().unwrap();
                return Some((this.0, this.1 - other.1));
            }
        }
        None
    }
}

impl<'t> Tower<'t> {
    fn find_root(&self) -> &str {
        self.data
            .iter()
            .filter(|&(_, node)| node.parent == None)
            .next()
            .unwrap()
            .0
    }

    fn balance(&self) -> (&str, i32) {
        fn balance_supporting<'a>(
            name: &'a str,
            parent_imbal: i32,
            tower: &'a Tower,
        ) -> Option<(&'a str, i32)> {
            // println!("{:?}", name);

            let this = &tower.data[name];
            let imbalance = this.imbalance(tower);

            Some(
                imbalance
                    .and_then(|(s, w)| balance_supporting(s, w, tower))
                    .unwrap_or((name, this.weight - parent_imbal)),
            )
        }

        let root = self.find_root();

        balance_supporting(root, 0, self).unwrap()
    }
}

/// Q2
fn part2(s: &str) -> i32 {
    let tower = to_tower(s);

    tower.balance().1
}



// Parser
#[derive(Debug, PartialEq)]
struct TowerParseNode<'a> {
    name: &'a str,
    weight: i32,
    supporting: Vec<&'a str>,
}

fn calc_sup_weight(name: &str, tower: &Tower) -> i32 {
    let n = &tower.data[name];
    if n.sup_weight > 0 {
        return n.sup_weight;
    }
    let ns = &n.supporting;
    if ns.len() > 0 {
        ns.iter()
            .fold(n.weight, |acc, &x| acc + calc_sup_weight(x, tower))
    } else {
        n.weight
    }
}

fn to_tower(s: &str) -> Tower {
    let parsed = parse_tower(s.as_bytes());

    if let nom::IResult::Done(_, nodes) = parsed {
        let mut tower = Tower {
            data: HashMap::new(),
        };

        for node in &nodes {
            tower.data.insert(
                node.name,
                TowerNode {
                    weight: node.weight,
                    supporting: node.supporting.clone(),
                    parent: None,
                    sup_weight: 0,
                },
            );
        }

        for node in &nodes {
            let name = node.name;
            // find parents
            for s in &node.supporting {
                tower.data.get_mut(s).unwrap().parent = Some(name);
            }
            // sub weights
            tower.data.get_mut(name).unwrap().sup_weight = calc_sup_weight(name, &tower);
        }

        tower
    } else {
        panic!("parse failed")
    }
}

named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));

named!(csv_vec(&[u8]) -> Vec<&str>, 
    separated_list!(is_a_s!(", "), map_res!(alpha, str::from_utf8))
);

named!(parse_supported(&[u8]) -> Vec<&str>,
    preceded!(is_a_s!(" ->"), csv_vec)
);

named!(parse_tower_row(&[u8]) -> TowerParseNode,
    do_parse!(
        name: map_res!(alpha, str::from_utf8) >>
        space >>
        weight: map_res!(parens, str::from_utf8) >>
        supp: opt!(complete!(parse_supported)) >>
        opt!(complete!(multispace)) >>
        
        (TowerParseNode{
            name: name,
            weight: weight.parse().unwrap(),
            supporting: match supp {
                Some(s) => { s }
                None => { Vec::new() }
            },
        })
    )
);

named!(parse_tower(&[u8]) -> Vec<TowerParseNode>, 
    many0!(parse_tower_row)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(part1(input), "tknk");
    }

    #[test]
    fn parser_test() {
        let input = "fwft (72)";

        let output = parse_tower_row(input.as_bytes());

        assert_eq!(
            output,
            nom::IResult::Done(
                "".as_bytes(),
                TowerParseNode {
                    name: "fwft",
                    weight: 72,
                    supporting: vec![],
                }
            )
        );
    }

    #[test]
    fn parser_test2() {
        let input = "-> ktlj, cntj, xhth";

        let output = parse_supported(input.as_bytes());

        assert_eq!(
            output,
            nom::IResult::Done("".as_bytes(), vec!["ktlj", "cntj", "xhth"])
        );
    }

    #[test]
    fn parser_test3() {
        let input = "fwft (72) -> ktlj, cntj, xhth";

        let output = parse_tower_row(input.as_bytes());

        assert_eq!(
            output,
            nom::IResult::Done(
                "".as_bytes(),
                TowerParseNode {
                    name: "fwft",
                    weight: 72,
                    supporting: vec!["ktlj", "cntj", "xhth"],
                }
            )
        );
    }

    #[test]
    fn sample2() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(part2(input), 60);
    }
}
