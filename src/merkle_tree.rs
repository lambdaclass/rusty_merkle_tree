use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Inner structure contains a binary tree stored in an array with double the size of the initial
// element count, parent, left child, right child access is implicit (check left_child_index,
// right_child_index, parent_index implementations).
pub struct MerkleTree {
    nodes: Vec<String>,
    pub root_index: Option<usize>,
    size: usize,
}

impl MerkleTree {
    // Transforms the vector of elements (to be inserted in the tree) to their respective hashes.
    // It also ensures that the vector size is a power of 2 (if the original vector does not have
    // enough elements, it will end up filled with copies of the last element
    fn hash_leaves<T>(input_elements: Vec<T>) -> Vec<String>
    where
        T: Hash + std::clone::Clone,
    {
        let mut hashed_input_elements: Vec<String> = Vec::new();
        for element in input_elements.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashed_input_elements.push(hasher.finish().to_string());
        }
        hashed_input_elements
    }

    fn complete_until_power_of_two(input_elements: Vec<String>) -> Vec<String> {
        let mut power_of_two_elements = input_elements;
        while !MerkleTree::is_power_of_two(power_of_two_elements.len()) {
            power_of_two_elements
                .push(power_of_two_elements[power_of_two_elements.len() - 1].clone());
        }
        power_of_two_elements
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

    pub fn nodes_count_equals(&self, size: u32) -> bool {
        self.nodes.len() == size as usize
    }

    pub fn has_size(&self, size: u32) -> bool {
        self.size as u32 == size
    }

    pub fn size_in_bytes(&self) -> usize {
        std::mem::size_of_val(self) + (self.nodes.len() * std::mem::size_of::<String>())
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
        let size = input_elements.len();
        let leaves = MerkleTree::complete_until_power_of_two(input_elements);
        let mut nodes = vec!["".to_string(); leaves.len()];
        for elem in leaves.iter() {
            nodes.push(elem.to_string());
        }
        let mut merkle_tree = MerkleTree {
            nodes,
            root_index: Some(1),
            size,
        };
        merkle_tree.build(merkle_tree.root_index.unwrap());
        merkle_tree
    }

    fn leaf_index_of(&self, elem: &str) -> Option<usize> {
        self.get_leaves()
            .iter()
            .position(|hashed_leaf| *hashed_leaf == elem)
    }

    fn node_index_of(&self, elem: &str) -> Option<usize> {
        self.nodes
            .iter()
            .position(|hash_in_nodes| *hash_in_nodes == elem)
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

    /// returns the merkle proof for the given element. If the element is not present in the tree it will return an empty proof
    pub fn proof<T>(&self, element: T) -> Vec<String>
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash = hasher.finish().to_string();
        match self.node_index_of(&hash) {
            Some(mut current) => {
                let mut proof = Vec::new();
                assert!(self.is_leaf(current));
                while current != self.root_index.unwrap() {
                    proof.push(self.nodes[MerkleTree::sibling_index(current)].to_string());
                    current = MerkleTree::parent_index(current);
                }
                proof
            }
            None => vec![],
        }
    }

    /// creates a new merkle tree with the added element
    pub fn add<T>(&self, element: T) -> MerkleTree
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        self.add_hashed(hasher.finish().to_string())
    }

    /// obtain a vector with the leaves of the merkle tree
    fn get_leaves(&self) -> Vec<String> {
        let leaves_start = self.nodes.len() / 2;
        Vec::from(&self.nodes[leaves_start as usize..(leaves_start as usize + self.size)])
    }

    pub fn add_hashed(&self, element: String) -> MerkleTree {
        let mut leaves = self.get_leaves();
        leaves.push(element);

        MerkleTree::new_from_hashed(leaves)
    }

    /// creates a new merkle tree without the value that was removed
    pub fn delete_element<T>(&self, element: T) -> Result<MerkleTree, String>
    where
        T: Hash,
    {
        let mut leaves_of_the_actual_tree = self.get_leaves();
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash = hasher.finish().to_string();
        if let Some(element_to_remove_index) = self.leaf_index_of(&hash) {
            leaves_of_the_actual_tree.remove(element_to_remove_index);
            Ok(MerkleTree::new_from_hashed(leaves_of_the_actual_tree))
        } else {
            Err("Element not present".to_string())
        }
    }
}

/// Verifies a given merkle proof is valid.
pub fn verify_tree_element<T>(tree: &MerkleTree, element: T, proof: Vec<String>) -> bool
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    element.hash(&mut hasher);
    let hashed_element = hasher.finish().to_string();
    let root_hash = tree.get_root_hash();
    let elem_index = tree.leaf_index_of(&hashed_element);

    if let Some(index) = elem_index {
        verify(hashed_element, index, proof, root_hash)
    } else {
        false
    }
}

fn verify(
    mut hashed_element: String,
    mut elem_index: usize,
    proof: Vec<String>,
    root_hash: String,
) -> bool {
    for elem in proof.iter() {
        if elem_index % 2 == 0 {
            hashed_element = MerkleTree::hash_nodes(hashed_element, elem.to_string());
        } else {
            hashed_element = MerkleTree::hash_nodes(elem.to_string(), hashed_element);
        }

        elem_index /= 2;
    }

    root_hash == hashed_element
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01_check_key_is_inside_tree() {
        let merkle = MerkleTree::new_from(vec!["hey", "hey2"]);
        let proof_vec = merkle.proof("hey".to_string());
        assert!(verify_tree_element(&merkle, "hey".to_string(), proof_vec));
    }

    #[test]
    fn test02_check_key_is_not_inside_tree() {
        let merkle = MerkleTree::new_from(vec!["hey", "hey2"]);
        let proof_vec = merkle.proof("hey3".to_string());
        assert!(!verify_tree_element(&merkle, "hey3".to_string(), proof_vec));
    }

    #[test]
    fn test03_building_a_merkle_tree_out_of_a_vector_of_size_not_a_power_of_two_should_build_it_with_a_power_of_two_node_count(
    ) {
        // Given a non power of two input vector size
        let input_elements = vec!["hey", "hey2", "hey3"];
        let input_elements_size: u32 = input_elements.len() as u32;
        let two: u32 = 2;
        let expected_nodes_count = two.pow(input_elements_size);

        // When building a Merkle tree with the input elements
        let tree = MerkleTree::new_from(input_elements);

        // Then the nodes count should be two to the power of the input elements count
        assert_eq!(tree.nodes_count_equals(expected_nodes_count), true);
    }

    #[test]
    fn test04_adding_an_element_to_a_two_to_the_power_of_k_elements_tree_should_duplicate_the_node_count(
    ) {
        let k: u32 = 3;
        let two: u32 = 2;
        let two_to_the_power_of_k = two.pow(k);
        let expected_node_count_before_adding = 2 * two_to_the_power_of_k;
        let expected_node_count_after_adding = 2 * expected_node_count_before_adding;
        let input_elements = (0..two_to_the_power_of_k)
            .map(|e| format!("{}", e))
            .collect();

        let tree = MerkleTree::new_from(input_elements);
        let tree = tree.add("hey");

        assert_eq!(tree.has_size(two_to_the_power_of_k + 1), true);
        assert!(tree.nodes_count_equals(expected_node_count_after_adding));
    }

    #[test]
    fn test05_deleting_an_element_to_a_two_to_the_power_of_k_elements_tree_should_maintain_the_node_count_and_reduce_the_tree_size(
    ) {
        let k: u32 = 3;
        let two: u32 = 2;
        let two_to_the_power_of_k = two.pow(k);
        let expected_node_count_before_removing = 2 * two_to_the_power_of_k;
        let expected_node_count_after_removing = expected_node_count_before_removing;
        let input_elements = (0..two_to_the_power_of_k)
            .map(|e| format!("{}", e))
            .collect();

        let tree = MerkleTree::new_from(input_elements);
        let tree = tree
            .delete_element(format!("{}", two_to_the_power_of_k - 1))
            .unwrap();

        assert_eq!(tree.has_size(two_to_the_power_of_k - 1), true);
        assert!(tree.nodes_count_equals(expected_node_count_after_removing));
    }

    #[test]
    fn test06_removing_an_element_to_a_two_to_the_power_of_k_plus_one_elements_tree_should_reduce_by_half_the_node_count(
    ) {
        let k: u32 = 3;
        let two: u32 = 2;
        let two_to_the_power_of_k_plus_one = two.pow(k) + 1;
        let expected_node_count_before_removing = 4 * (two_to_the_power_of_k_plus_one - 1);
        let expected_node_count_after_removing = expected_node_count_before_removing / 2;
        let input_elements = (0..two_to_the_power_of_k_plus_one)
            .map(|e| format!("{}", e))
            .collect();

        let tree = MerkleTree::new_from(input_elements);
        let tree_deleted = tree
            .delete_element(format!("{}", two_to_the_power_of_k_plus_one - 1))
            .unwrap();

        assert_eq!(
            tree_deleted.has_size(two_to_the_power_of_k_plus_one - 1),
            true
        );
        assert!(tree_deleted.nodes_count_equals(expected_node_count_after_removing));
    }

    #[test]
    fn test07_adding_an_element_to_a_two_to_the_power_of_k_plus_one_elements_tree_should_maintain_the_node_count(
    ) {
        let k: u32 = 3;
        let two: u32 = 2;
        let two_to_the_power_of_k_plus_one = two.pow(k) + 1;
        let expected_node_count_before_adding = 4 * (two_to_the_power_of_k_plus_one - 1);
        let expected_node_count_after_adding = expected_node_count_before_adding;
        let input_elements = (0..two_to_the_power_of_k_plus_one)
            .map(|e| format!("{}", e))
            .collect();

        let tree = MerkleTree::new_from(input_elements);
        let tree = tree.add(format!("{}", two_to_the_power_of_k_plus_one));

        assert_eq!(tree.has_size(two_to_the_power_of_k_plus_one + 1), true);
        assert!(tree.nodes_count_equals(expected_node_count_after_adding));
    }

    #[test]
    fn test08_the_memory_space_occupied_grows_linearly() {
        let k: u32 = 10;
        let two: u32 = 2;
        let two_to_the_power_of_k = two.pow(k);
        let two_to_the_power_of_k_plus_one = two.pow(k + 1);
        let k_input_elements = (0..two_to_the_power_of_k)
            .map(|e| format!("{}", e))
            .collect();
        let k_plus_one_input_elements = (0..two_to_the_power_of_k_plus_one)
            .map(|e| format!("{}", e))
            .collect();

        let k_tree = MerkleTree::new_from(k_input_elements);
        let k_plus_one_tree = MerkleTree::new_from(k_plus_one_input_elements);

        // std::mem::size_of_val(&k_tree) is subtracted because it is duplicated in the multiplication
        assert_eq!(
            (2 * k_tree.size_in_bytes()) - std::mem::size_of_val(&k_tree),
            k_plus_one_tree.size_in_bytes()
        );
    }
}
