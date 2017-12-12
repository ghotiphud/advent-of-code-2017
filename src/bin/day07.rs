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

/// One program at the bottom supports the entire tower. It's holding a
/// large disc, and on the disc are balanced several more sub-towers.
/// At the bottom of these sub-towers, standing on the bottom disc,
/// are other programs, each holding their own disc, and so on. At the
/// very tops of these sub-sub-sub-...-towers, many programs stand simply
/// keeping the disc below them balanced but with no disc of their own.
///
/// You offer to help, but first you need to understand the structure of
/// these towers. You ask each program to yell out their name, their
/// weight, and (if they're holding a disc) the names of the programs
/// immediately above them balancing on that disc. You write this
/// information down (your puzzle input). Unfortunately, in their panic,
/// they don't do this in an orderly fashion; by the time you're done,
/// you're not sure which program gave which information.
///
/// For example, if your list is the following:
///
/// pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)
///
/// ...then you would be able to recreate the structure of the towers that
/// looks like this:
///
///                 gyxo
///               /
///          ugml - ebii
///        /      \
///       |         jptl
///       |
///       |         pbga
///      /        /
/// tknk --- padx - havc
///      \        \
///       |         qoyq
///       |
///       |         ktlj
///        \      /
///          fwft - cntj
///               \
///                 xhth
///
/// In this example, tknk is at the bottom of the tower (the bottom program),
/// and is holding up ugml, padx, and fwft. Those programs are, in turn,
/// holding up other programs; in this example, none of those programs are
/// holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)
///
/// Before you're ready to help them, you need to make sure your information
/// is correct. What is the name of the bottom program?
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

/// Apparently, one program has the wrong weight, and until it's fixed,
/// they're stuck here.
///
/// For any program holding a disc, each program standing on that disc
/// forms a sub-tower. Each of those sub-towers are supposed to be the
/// same weight, or the disc itself isn't balanced. The weight of a tower
/// is the sum of the weights of the programs in that tower.
///
/// In the example above, this means that for ugml's disc to be balanced,
/// gyxo, ebii, and jptl must all have the same weight, and they do: 61.
///
/// However, for tknk to be balanced, each of the programs standing on its
/// disc and all programs above it must each match. This means that the
/// following sums must all be the same:
///
/// ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
/// padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
/// fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
/// As you can see, tknk's disc is unbalanced: ugml's stack is heavier
/// than the other two. Even though the nodes above ugml are balanced,
/// ugml itself is too heavy: it needs to be 8 units lighter for its stack
/// to weigh 243 and keep the towers balanced. If this change were made,
/// its weight would be 60.
///
/// Given that exactly one program is the wrong weight, what would its
/// weight need to be to balance the entire tower?
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
