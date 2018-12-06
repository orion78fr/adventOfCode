const PUZZLE: &str = include_str!("../input");

fn main() {
    let output = react(PUZZLE.chars());

    // For my input : 9704
    println!("{}", output.len());


    let (character_to_remove, length) = (b'a'..b'z' + 1)
        .map(|c| c as char)
        .map(|character_to_remove|
            (character_to_remove, react(output.chars()
                .filter(|x| !x.eq_ignore_ascii_case(&character_to_remove)))
                .len()))
        .min_by(|(_, len), (_, len2)| len.cmp(len2))
        .unwrap();

    // For my input : m and 6942
    println!("Char {} length {}", character_to_remove, length);
}

fn react(polymer: impl Iterator<Item=char>) -> String {
    let mut output = String::new();

    for character in polymer {
        let last_char = output.chars().last();
        if output.is_empty() || !opposite_capitalisation(character, last_char.unwrap()) {
            output += &character.to_string();
        } else {
            let len = output.len();
            output.truncate(len - 1);
        }
    }

    return output;
}

fn opposite_capitalisation(left: char, right: char) -> bool {
    left.eq_ignore_ascii_case(&right) && !left.eq(&right)
}