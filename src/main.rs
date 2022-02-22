use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

struct MerkleTree {
    pub nodes: Vec<MerkleTreeNode>,
    pub root_index: Option<usize>,
}

struct MerkleTreeNode {
    pub hash: String,
}

impl Clone for MerkleTreeNode {
    fn clone(&self) -> MerkleTreeNode {
        MerkleTreeNode {
            hash: self.hash.to_string(),
        }
    }
}

impl MerkleTree {
    fn build_hashes<T>(input_elements: Vec<T>) -> Vec<String>
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

    fn propagate_hashes(nodes: &mut Vec<MerkleTreeNode>, node_index: usize) {
        MerkleTree::propagate_hashes(nodes, MerkleTree::left_child_index(node_index));
        MerkleTree::propagate_hashes(nodes, MerkleTree::right_child_index(node_index));
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
        let input_elements = MerkleTree::build_hashes(input_elements);
        let mut nodes: Vec<MerkleTreeNode> = Vec::with_capacity(input_elements.len());
        for elem in input_elements.iter() {
            nodes.push(MerkleTreeNode {
                hash: elem.to_string(),
            });
        }

        MerkleTree::propagate_hashes(&mut nodes, 1);
        let mut input_elements = VecDeque::from(input_elements);
        while input_elements.len() >= 2 {
            let left_hash = input_elements.pop_front().unwrap();
            let right_hash = input_elements.pop_front().unwrap();
            nodes.push(MerkleTreeNode {
                hash: left_hash.to_string(),
            });
            nodes.push(MerkleTreeNode {
                hash: right_hash.to_string(),
            });
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
