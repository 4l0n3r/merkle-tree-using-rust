use std::fs;
use sha2::{Sha256, Digest};

fn hash_pair(left: &[u8;32], right: &[u8;32]) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(&left);
    hasher.update(&right);
    hasher.finalize().into()
}

fn make_even(mut hashes:Vec<[u8;32]>) -> Vec<[u8;32]> {
    if (hashes.len() % 2 != 0) {
        hashes.push(hashes[hashes.len()-1]);
    }
    hashes
}


fn generate_merkle_tree(hashes: &Vec::<i32> ) -> Vec<Vec<[u8;32]>> {
    if hashes.len() == 0 {
        return vec![];
    }

    let mut leaf_nodes:Vec<[u8;32]> = vec![];
    for element in hashes {
        let mut hasher = Sha256::new();
        hasher.update(element.to_string());
        leaf_nodes.push(hasher.finalize().into());
    }

    leaf_nodes = make_even(leaf_nodes);

    let mut current_branch = leaf_nodes.clone();
    let mut tree:Vec<Vec<[u8;32]>> = vec![];
    while current_branch.len() != 1 {
        current_branch = make_even(current_branch);
        let mut combined_hashes:Vec<[u8;32]> = vec![];
        for i in (0..current_branch.len()).step_by(2) {
           let combined_hash = hash_pair(&current_branch[i],&current_branch[i+1]);
           combined_hashes.push(combined_hash);
        }
        tree.push(current_branch);
        current_branch = combined_hashes;
    }
    return tree;
}

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let leaf_nodes:Vec<i32> = content.split(' ').map(|s| s.trim().parse::<i32>().expect("Invalid number")).collect();
    println!("{:?}",leaf_nodes);
    let tree:Vec<Vec<[u8;32]>> = generate_merkle_tree(&leaf_nodes);
//     println!("{:?}",tree);
}