extern crate itertools;

use std::collections::HashMap;

use itertools::Itertools;

const PUZZLE: &str = include_str!("../input");

fn main() {
    let mut match2 = 0;
    let mut match3 = 0;

    PUZZLE.lines().for_each(|id| {
        let mut letter_occurrences = HashMap::with_capacity(26);
        for c in id.chars() {
            *letter_occurrences.entry(c).or_insert(0) += 1;
        }

        if letter_occurrences.values().any(|&v| v == 2) {
            match2 += 1;
        }

        if letter_occurrences.values().any(|&v| v == 3) {
            match3 += 1;
        }
    });

    // For my input : 4920
    println!("Checksum : {}", match2 * match3);

    let (matching_id1, matching_id2) = PUZZLE.lines()
        .tuple_combinations()
        .find(|(r, l)| {
            r.chars().zip(l.chars())
                .filter(|(r, l)| {
                    return r != l;
                })
                .count() == 1
        }).unwrap();

    let ((diff_idx, _), _) = matching_id1.char_indices()
        .zip(matching_id2.chars())
        .find(|((_, r), l)| {
            r != l
        }).unwrap();

    // For my input : fonbwmjquwtapeyzikghtvdxl
    println!("{}{}", &matching_id1[0..diff_idx], &matching_id1[diff_idx + 1..matching_id1.len()])
}
