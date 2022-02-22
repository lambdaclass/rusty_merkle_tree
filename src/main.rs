use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    let merkle = MerkleTree::new_from(vec!("hey", "hey2"));
    merkle.proof(0);
}
