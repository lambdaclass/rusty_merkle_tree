use merkle_tree::merkle_tree::verify;
use merkle_tree::merkle_tree::MerkleTree;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let merkle = MerkleTree::new_from(vec!["hey", "hey2"]);
    println!("Elementos antes de agregar: {}", merkle.size);
    let mut merkle_added = merkle.add("hey3".to_string());
    println!("Elementos despues de agregar: {}", merkle_added.size);
    let merkle_deleted = merkle_added.delete_element("hey".to_string()).unwrap();
    println!("Elementos despues de quitar: {}", merkle_deleted.size);
}
