extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

const PUZZLE: &str = include_str!("../input");

#[derive(Parser)]
#[grammar = "dependency.pest"]
pub struct DependencyParser;

fn main() {
    let mut deps: Vec<_> = PUZZLE.lines()
        .map(|dep| {
            let mut parse_res = DependencyParser::parse(Rule::dependency, dep).unwrap()
                .next().unwrap().into_inner();

            let before = parse_res.next().unwrap().as_str();
            let after = parse_res.next().unwrap().as_str();

            (before, after)
        })
        .collect();

    let mut res = String::new();

    while !deps.is_empty() {
        for c in b'A'..b'Z' + 1 {
            let c = c as char;
            let c = c.to_string();

            if deps.iter().any(|(before, _)| *before == c)
                && !deps.iter().any(|(_, after)| *after == c) {
                res += &c;
                deps = deps.iter().filter(|(before, _)| *before != c).map(|p| *p).collect();
                break;
            }
        }
    }

    // For my input : GRTAHKLQVYWXMUBCZPIJFEDNSO
    res+= &(b'A'..b'Z' + 1)
        .map(|c| c as char)
        .map(|c| c.to_string())
        .find(|c| !res.contains(c))
        .unwrap();

    println!("{}", res);
}