use std::fs;
use std::collections::HashSet;

fn main() {
    let file_content = fs::read_to_string("input").unwrap();

    let mut frequency = 0;

    file_content.lines()
        .for_each(|change| {
            let change = parse_freq_change(change);
            frequency += change;
        });

    println!("Frequency is {}", frequency);

    let dupe = find_first_dupe_freq(file_content);
    println!("First duplicated frequency is {}", dupe)
}

fn find_first_dupe_freq(file_content: String) -> i32 {
    let mut set = HashSet::new();
    let mut frequency = 0;
    set.insert(0);

    loop {
        for change in file_content.lines() {
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
