use std::collections::HashMap;

const PUZZLE: &str = include_str!("../input");

fn main() {
    let values: Vec<_> = PUZZLE.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let (_, metadata_sum, node_value) = sum_tree_metadata(&values[..]);

    // For my input : 44893
    println!("Metadata sum {}", metadata_sum);
    // For my input : 27433
    println!("Value {}", node_value);
}

fn sum_tree_metadata(subtree: &[i32]) -> (usize, i32, i32) {
    let subtrees = subtree[0];
    let metadata_count = subtree[1] as usize;

    let mut idx = 2;
    let mut sum = 0;
    let mut subtree_read = 0;
    let mut subtree_values = HashMap::new();

    while subtree_read < subtrees {
        let (end_idx, subtree_sum, subtree_value) = sum_tree_metadata(&subtree[idx..]);
        idx += end_idx;
        sum += subtree_sum;
        subtree_read += 1;
        subtree_values.insert(subtree_read, subtree_value);
    }

    sum += &subtree[idx..idx + metadata_count]
        .iter().sum();

    let tree_value : i32 = if subtrees == 0 {
        sum
    } else {
        subtree[idx..idx + metadata_count]
        .iter().map(|s| subtree_values.get(s).unwrap_or(&0)).sum()
    };

    return (idx + metadata_count, sum, tree_value);
}