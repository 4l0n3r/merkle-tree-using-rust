use std::fs;
use sha2::{Sha256, Digest};

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct Node<'a> {
    hash: &'a [u8;32],
    direction: Direction
}

fn hash_pair(left: &[u8;32], right: &[u8;32]) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(&left);
    hasher.update(&right);
    hasher.finalize().into()
}

fn index(target:&[u8;32], vector:&Vec<[u8;32]>) -> usize {
    return vector.iter().position(|x| x == target).unwrap()
}

fn make_even(mut hashes:Vec<[u8;32]>) -> Vec<[u8;32]> {
    if hashes.len() % 2 != 0 {
        hashes.push(hashes[hashes.len()-1]);
    }
    hashes
}

fn get_direction(target:&[u8;32], tree:&Vec<Vec<[u8;32]>>, level:usize) -> Direction {
    let location = index(target,&tree[level]);
    if location % 2 == 0 {
        Direction::LEFT
    } else {
        Direction::RIGHT
    }
}

fn generate_merkle_proof<'a>(hash:&'a [u8;32], tree:&'a Vec<Vec<[u8;32]>>) -> Vec<Node<'a>> {

    if tree.is_empty() {
        return vec![];
    }


    let mut hash_index = index(hash,&tree[0]);
    let mut merkle_proof:Vec<Node> = vec![];
    let direction:Direction = get_direction(hash,tree,0);
    let node = Node {hash, direction};

    merkle_proof.push(node);

    for level in 0..tree.len()-1 {
        let is_left_child = hash_index % 2 == 0 ;
        let sibling_direction:Direction;
        let sibling_index;
        if is_left_child {
            sibling_direction =  Direction::RIGHT;
            sibling_index = hash_index + 1;
        }
        else {
            sibling_direction = Direction::LEFT;
            sibling_index = hash_index - 1;
        }

        let sibling_node = Node {hash: &tree[level][sibling_index], direction:sibling_direction};
        merkle_proof.push(sibling_node);
        hash_index /= 2;
    }
    return merkle_proof;
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
    tree.push(current_branch);
    return tree;
}

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let leaf_nodes:Vec<i32> = content.split(' ').map(|s| s.trim().parse::<i32>().expect("Invalid number")).collect();
    println!("{:?}",leaf_nodes);

    let tree:Vec<Vec<[u8;32]>> = generate_merkle_tree(&leaf_nodes);
    println!("{:?}",tree);

    let mut hasher = Sha256::new();
    hasher.update(&leaf_nodes[0].to_string());
    let target_hash = hasher.finalize().into();

    let merkle_proof: Vec<Node> = generate_merkle_proof(&target_hash, &tree);
    println!("{:?}",merkle_proof);

}