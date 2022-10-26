use sparse_merkle_tree::branch::BranchKey;
use {sparse_merkle_tree::*};
use sparse_merkle_tree::merge::hash_base_node;
use sparse_merkle_tree::tree::SparseMerkleTree as OldTree;
use sparse_merkle_tree::trie_tree::SparseMerkleTree as TrieMerkleTree;
#[allow(clippy::upper_case_acronyms)]
type SMT = OldTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
type SMT_NEW = TrieMerkleTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
use sparse_merkle_tree::merge::MergeValue::{MergeWithZero, ShortCut};
//use sparse_merkle_tree::merge::merge_with_zero;
use sparse_merkle_tree::merge::merge;
use sparse_merkle_tree::traits::StoreReadOps;

fn main() {
    //let mut vec = Vec::new();
    let mut tree = SMT::default();
    tree.store_mut().enable_counter(true);

    let mut trie_tree = SMT_NEW::default();
    trie_tree.store_mut().enable_counter(true);

    let mut key = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
    let value = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();

    tree.update(key.into(), value);
    trie_tree.update(key.into(), value);

    let root = trie_tree.root().clone();

    key = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];


    tree.update(key.into(), value);
    trie_tree.update(key.into(), value);

    let branch_key =  BranchKey::new(255, key.into());

    if let Some(branch) = tree.store().get_branch(&branch_key).unwrap() {
        println!("old branch: {:?}", branch);
    }

    if let Some(branch) = trie_tree.store().get_branch(&branch_key).unwrap() {
        println!("new branch: {:?}", branch);
    }

    println!("root: {:?}, new_root: {:?}", tree.root(), trie_tree.root());

    assert_eq!(tree.root(), trie_tree.root());

}