use std::collections::hash_map::DefaultHasher;
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

    fn build(&mut self, parent_index: usize) {
        if self.is_leaf(parent_index) {
            return;
        }
        let left_child_index = MerkleTree::left_child_index(parent_index);
        let right_child_index = MerkleTree::left_child_index(parent_index);
        self.build(left_child_index);
        self.build(right_child_index);
        self.nodes[parent_index] = self.hash_nodes(
            self.nodes[left_child_index].clone(),
            self.nodes[right_child_index].clone(),
        );
    }

    fn hash_nodes(&self, left_child: String, right_child: String) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(left_child.as_bytes());
        hasher.write(right_child.as_bytes());
        hasher.finish().to_string()
    }

    fn is_leaf(&self, node_index: usize) -> bool {
        (node_index >= (self.nodes.len()/2)) && node_index < self.nodes.len()
    }

    fn left_child_index(parent_index: usize) -> usize {
        parent_index * 2
    }

    fn right_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 1
    }

    fn sibling_index(node_index: usize) -> usize {
        if node_index % 2 == 0 {
            node_index + 1
        } else {
            node_index - 1
        }
    }

    fn parent_index(node_index) -> usize {
        node_index / 2
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
        let mut merkle_tree = MerkleTree {
            nodes: nodes,
            root_index: Some(1),
        };
        merkle_tree.build(merkle_tree.root_index.unwrap());

        merkle_tree
    }

    pub fn proof(&self, elem_index: usize) -> Vec<String> {
        let current = elem_index;
        let proof = Vec::new();
        assert!(self.is_leaf(current));
        while current != self.root_index.unwrap() {
            proof.push(self.nodes[MerkleTree::sibling_index(current)]);
            current = MerkleTree::parent_index(current);
        }

        proof
    }

    pub fn verify() {}
}

fn main() {}
