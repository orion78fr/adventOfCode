use std::collections::HashMap;
use std::collections::hash_map::Entry;

const PUZZLE: &str = include_str!("../input");

fn main() {
    let mut match2 = 0;
    let mut match3 = 0;
    let mut occurrences = HashMap::new();

    PUZZLE.lines().for_each(|id| {
        let mut letter_occurrences = HashMap::new();
        for c in id.chars() {
            match letter_occurrences.entry(c) {
                Entry::Occupied(o) => *o.into_mut() += 1,
                Entry::Vacant(v) => { v.insert(1); }
            };
        }

        if letter_occurrences.iter().any(|(_, v)| *v == 2) {
            match2 += 1;
        }

        if letter_occurrences.iter().any(|(_, v)| *v == 3) {
            match3 += 1;
        }

        occurrences.insert(id, letter_occurrences);
    });

    // For my input : 4920
    println!("Checksum : {}", match2 * match3);

    let mut best_match : Option<(i32, i32, i32)> = None();


}
