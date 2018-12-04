extern crate itertools;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use itertools::Itertools;
use pest::Parser;

#[derive(Parser)]
#[grammar = "claim.pest"]
pub struct ClaimParser;

const PUZZLE: &str = include_str!("../input");

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let mut claim_map = HashMap::new();
    let mut non_overlapping_claims = HashMap::new();

    PUZZLE.lines()
        .map(decode_claim)
        .for_each(|Claim { id, x, y, w, h }| {
            (x..x + w).cartesian_product(y..y + h)
                .for_each(|(i, j)| {
                    match claim_map.entry((i, j)) {
                        Entry::Occupied(mut e) => {
                            let (first_id, occ) = e.get_mut();
                            *occ += 1;
                            non_overlapping_claims.insert(*first_id, false);
                            non_overlapping_claims.insert(id, false);
                        }
                        Entry::Vacant(e) => {
                            e.insert((id, 1));
                            non_overlapping_claims.entry(id).or_insert(true);
                        }
                    }
                })
        });

    let overlap_tiles = claim_map.values()
        .filter(|(_, v)| *v > 1)
        .count();

    // For my input : 101196
    println!("Overlapping tiles : {}", overlap_tiles);

    let non_overlapping = non_overlapping_claims.iter()
        .find(|(_, state)| **state).unwrap().0;

    println!("Non overlapping claim : {}", non_overlapping);
}

fn decode_claim(claim: &str) -> Claim {
    let parsed_claim = ClaimParser::parse(Rule::line, claim).unwrap().next().unwrap();

    let numbers: Vec<i32> = parsed_claim.into_inner()
        .filter(|x| match x.as_rule() {
            Rule::number => true,
            _ => false
        })
        .map(|num| num.into_span().as_str().parse().unwrap())
        .collect();

    Claim {
        id: numbers[0],
        x: numbers[1],
        y: numbers[2],
        w: numbers[3],
        h: numbers[4]
    }
}