use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

struct MerkleTree {
    pub nodes: Vec<String>,
    pub root_index: Option<usize>,
}

impl MerkleTree {
    fn hash_leaves<T>(input_elements: Vec<T>) -> Vec<String>
    where
        T: Hash,
    {
        let mut hashed_input_elements: Vec<String> = Vec::new();
        let mut hasher = DefaultHasher::new();
        for element in input_elements.iter() {
            element.hash(&mut hasher);
            hashed_input_elements.push(hasher.finish().to_string());
        }
        hashed_input_elements
    }

    fn build(nodes: &mut Vec<String>, node_index: usize) {
        MerkleTree::build(nodes, MerkleTree::left_child_index(node_index));
        MerkleTree::build(nodes, MerkleTree::right_child_index(node_index));
    }

    fn left_child_index(parent_index: usize) -> usize {
        parent_index * 2
    }

    fn right_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 1
    }

    pub fn new_from<T>(input_elements: Vec<T>) -> Self
    where
        T: Hash,
    {
        let input_elements = MerkleTree::hash_leaves(input_elements);
        let mut nodes = Vec::with_capacity(input_elements.len());
        for elem in input_elements.iter() {
            nodes.push(elem.to_string());
        }

        MerkleTree::build(&mut nodes, 1);
        let mut input_elements = VecDeque::from(input_elements);
        while input_elements.len() >= 2 {
            let left_hash = input_elements.pop_front().unwrap();
            let right_hash = input_elements.pop_front().unwrap();
            nodes.push(left_hash.to_string());
            nodes.push(right_hash.to_string());
            input_elements.push_back(left_hash + &right_hash);
        }

        MerkleTree {
            nodes: nodes.clone(),
            root_index: Some(nodes.len() - 1),
        }
    }

    pub fn proof() {}

    pub fn verify() {}
}

fn main() {}
