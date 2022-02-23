use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Inner structure contains a binary tree stored in an array with double the size of the initial
// element count, parent, left child, right child access is implicit (check left_child_index,
// right_child_index, parent_index implementations).
pub struct MerkleTree {
    nodes: Vec<String>,
    pub root_index: Option<usize>,
    pub size: usize,
}

impl MerkleTree {
    // Transforms the vector of elements (to be inserted in the tree) to their respective hashes.
    // It also ensures that the vector size is a power of 2 (if the original vector does not have
    // enough elements, it will end up filled with copies of the last element
    fn hash_leaves<T>(mut input_elements: Vec<T>) -> Vec<String>
    where
        T: Hash + std::clone::Clone,
    {
        while !MerkleTree::is_power_of_two(input_elements.len()) {
            input_elements.push(input_elements[input_elements.len() - 1].clone());
        }
        let mut hashed_input_elements: Vec<String> = Vec::new();
        for element in input_elements.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashed_input_elements.push(hasher.finish().to_string());
        }
        hashed_input_elements
    }

    // recrusively computes the hash stored in the node given by parent_index by computing its
    // children hashes and then its corresponding hash
    fn build(&mut self, parent_index: usize) {
        if self.is_leaf(parent_index) {
            return;
        }
        let left_child_index = MerkleTree::left_child_index(parent_index);
        let right_child_index = MerkleTree::right_child_index(parent_index);
        self.build(left_child_index);
        self.build(right_child_index);
        self.nodes[parent_index] = MerkleTree::hash_nodes(
            self.nodes[left_child_index].clone(),
            self.nodes[right_child_index].clone(),
        );
    }

    fn hash_nodes(left_child: String, right_child: String) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(left_child.as_bytes());
        hasher.write(right_child.as_bytes());
        hasher.finish().to_string()
    }

    fn is_leaf(&self, node_index: usize) -> bool {
        (node_index >= (self.nodes.len() / 2)) && node_index < self.nodes.len()
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

    fn parent_index(node_index: usize) -> usize {
        node_index / 2
    }

    fn new_from_hashed(input_elements: Vec<String>) -> Self {
        let mut nodes = vec!["".to_string() ; input_elements.len()];
        for elem in input_elements.iter() {
            nodes.push(elem.to_string());
        }
        let mut merkle_tree = MerkleTree {
            nodes,
            root_index: Some(1),
            size: input_elements.len(),
        };
        merkle_tree.build(merkle_tree.root_index.unwrap());
        merkle_tree
    }

    fn is_power_of_two(x: usize) -> bool {
        (x != 0) && ((x & (x - 1)) == 0)
    }
      
    pub fn new_from<T>(input_elements: Vec<T>) -> Self
    where
        T: Hash + std::clone::Clone,
    {
        let input_elements = MerkleTree::hash_leaves(input_elements);
        MerkleTree::new_from_hashed(input_elements)
    }

    pub fn get_root_hash(&self) -> String { 
        self.nodes[self.root_index.unwrap()].to_string()
    }

    // returns the index of the element (if present) in the leaves array
    fn leaf_index_of(&self, elem: &String) -> Option<usize> {
        for i in (self.nodes.len()/2)..self.nodes.len() {
            if elem == &self.nodes[i] {
                return Some(i - self.nodes.len() / 2);
            }
        }

        None
    }

    /// returns the merkle proof for the element given its index in the leaves array. If the element is not present in the tree it will return an empty proof
    pub fn proof(&self, elem_index: usize) -> Vec<String> {
        let mut current = elem_index + self.nodes.len() / 2;
        let mut proof = Vec::new();
        assert!(self.is_leaf(current));
        while current != self.root_index.unwrap() {
            proof.push(self.nodes[MerkleTree::sibling_index(current)].to_string());
            current = MerkleTree::parent_index(current);
        }
        proof
    }

    /// creates a new merkle tree with the element added
    pub fn add<T>(&self, element: T) -> MerkleTree 
        where T: Hash 
    {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        self.add_hashed(hasher.finish().to_string())
    }

    /// creates a new merkle tree from the hash of a new index
    fn get_leaves(&self) -> Vec<String> {
        let leaves_start = self.nodes.len() / 2;
        let leaves = Vec::from(&self.nodes[leaves_start as usize..(leaves_start as usize + self.size)]);
        leaves
    }

    pub fn add_hashed(&self, element: String) -> MerkleTree 
    {
        let mut leaves = self.get_leaves();
        leaves.push(element);

        MerkleTree::new_from_hashed(leaves)
    }

    /// creates a new merkle tree from self with element removed
    pub fn delete_element<T>(&mut self, element: T) -> Result<MerkleTree, String>
    where 
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash = hasher.finish().to_string();
        if let Some(element_to_remove_index) = self.leaf_index_of(&hash) {
            self.nodes.remove(element_to_remove_index + self.nodes.len() / 2);
            self.size -= 1;
        } else {
            return Err("Element not present".to_string());
        }
        Ok(MerkleTree::new_from_hashed(self.get_leaves()))
    }
}

/// Verifies a given merkle proof is valid.
pub fn verify(element: String, mut elem_index: usize, proof: Vec<String>, root_hash: String) -> bool {
    let mut hash = element;
    for elem in proof.iter() {
        if elem_index % 2 == 0 {
            hash = MerkleTree::hash_nodes(hash, elem.to_string());
        } else {
            hash = MerkleTree::hash_nodes(elem.to_string(), hash);
        }

        elem_index /= 2;
    }

    root_hash == hash
}
