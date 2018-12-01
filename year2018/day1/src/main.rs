use std::fs;

fn main() {
    let file_content = fs::read_to_string("input").unwrap();

    let mut frequency = 0;

    file_content.lines()
        .for_each(|change| {
            let change = parse_freq_change(change);
            frequency += change;
        });

    println!("Frequency is {}", frequency);
}

fn parse_freq_change(freq_change : &str) -> i32 {
    freq_change.parse().unwrap()
}