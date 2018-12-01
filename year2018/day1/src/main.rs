use std::collections::HashSet;

const PUZZLE: &str = include_str!("../input");

fn main() {
    let frequency : i32 = PUZZLE.lines()
        .map(parse_freq_change)
        .sum();

    // For my input : 459
    println!("Frequency is {}", frequency);

    let dupe = find_first_dupe_freq();
    // For my input : 65474
    println!("First duplicated frequency is {}", dupe)
}

fn find_first_dupe_freq() -> i32 {
    let mut set = HashSet::new();
    let mut frequency = 0;
    set.insert(0);

    loop {
        for change in PUZZLE.lines() {
            let change = parse_freq_change(change);
            frequency += change;

            if set.contains(&frequency) {
                return frequency;
            } else {
                set.insert(frequency);
            }
        }
    }
}

fn parse_freq_change(freq_change: &str) -> i32 {
    freq_change.parse().unwrap()
}
