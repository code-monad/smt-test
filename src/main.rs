use sparse_merkle_tree::branch::BranchKey;
use {sparse_merkle_tree::*};
use sparse_merkle_tree::merge::hash_base_node;
#[allow(clippy::upper_case_acronyms)]
type SMT = SparseMerkleTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
type SMT_NEW = TrieMerkleTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
use sparse_merkle_tree::merge::MergeValue::{MergeWithZero, ShortCut};
use sparse_merkle_tree::merge::merge_with_zero;
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


    key = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];


    tree.update(key.into(), value);
    trie_tree.update(key.into(), value);
    println!("root: {:?}, new_root: {:?}", tree.root(), trie_tree.root())

}