const PUZZLE: &str = include_str!("../input");

fn main() {
    let values: Vec<_> = PUZZLE.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let (_, metadata_sum) = sum_tree_metadata(&values[..]);

    // For my input : 44893
    println!("Metadata sum {}", metadata_sum);

    
}

fn sum_tree_metadata(subtree: &[i32]) -> (usize, i32) {
    let subtrees = subtree[0];
    let metadata_count = subtree[1] as usize;

    let mut idx = 2;
    let mut sum = 0;
    let mut subtree_read = 0;

    while subtree_read < subtrees {
        let (end_idx, subtree_sum) = sum_tree_metadata(&subtree[idx..]);
        idx += end_idx;
        sum += subtree_sum;
        subtree_read += 1;
    }

    sum += &subtree[idx..idx + metadata_count]
        .iter().sum();

    return (idx + metadata_count, sum);
}