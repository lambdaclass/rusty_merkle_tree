use merkle_tree::merkle_tree::MerkleTree;
use merkle_tree::merkle_tree::verify;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut merkle = MerkleTree::new_from(vec!("hey", "hey2"));
   
    let proof_vec = merkle.proof(0);
    let mut hasher = DefaultHasher::new();
    "hey3".hash(&mut hasher);
    let hash = hasher.finish().to_string();
    println!("{}", verify(hash, 0, proof_vec, merkle.get_root_hash()));

    let mut hasher = DefaultHasher::new();
    "hey".hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let proof_vec = merkle.proof(0);
    println!("{}", verify(hash, 0, proof_vec, merkle.get_root_hash()));

    let mut hasher = DefaultHasher::new();
    "hey2".hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let proof_vec = merkle.proof(1);
    println!("{}", verify(hash, 1, proof_vec, merkle.get_root_hash()));

    let mut hasher = DefaultHasher::new();
    "hey3".hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let new_merkle = merkle.add_hashed(hash.clone());
    let proof_vec = new_merkle.proof(2);
    println!("{}", new_merkle.size);
    println!("{}", verify(hash, 2, proof_vec, new_merkle.get_root_hash()));

    let new_merkle = merkle.delete_element("hey".to_string());
    println!("{}", new_merkle.size);
}
