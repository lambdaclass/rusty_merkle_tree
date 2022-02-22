use merkle_tree::merkle_tree::MerkleTree;
use merkle_tree::merkle_tree::verify;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let merkle = MerkleTree::new_from(vec!("hey", "hey2"));
    let proof_vec = merkle.proof(0);
    let mut hasher = DefaultHasher::new();
    "hey".hash(&mut hasher);
    let hash = hasher.finish().to_string();
    println!("{}", verify(hash, 0, proof_vec, merkle.get_root_hash()));
}
