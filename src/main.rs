use std::ptr::hash;
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

    key = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].into();


    tree.update(key.into(), value);
    trie_tree.update(key.into(), value);


    key = [8,0,0,8,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,10].into();
    let mut key1 = H256::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let mut key2 = H256::from(key);


    tree.update(key.into(), value);
    trie_tree.update(key.into(), value);

    let height_vec = [0,1, 2,247, 248, 249, 250];


    for height in height_vec {
        println!("height: {}, is right:{}", height, key2.is_right(height));
        let branch_key = BranchKey::new( height,  key2.parent_path(height));

        if let Some(branch) = tree.store().get_branch(&branch_key).unwrap() {
            println!("old_branch: {:?}", branch);
            println!("hash left: {:?}\n hash right: {:?}", branch.left.hash::<blake2b::Blake2bHasher>(), branch.right.hash::<blake2b::Blake2bHasher>());
            println!("merge: {:?}", merge::<blake2b::Blake2bHasher>(height, &key.into(), &branch.left, &branch.right));
        }

        if let Some(branch) = trie_tree.store().get_branch(&branch_key).unwrap() {
            println!("new branch: {:?}", branch);
            let left_key = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let right_key = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
            println!("base_left: {:?}, base_right: {:?}", hash_base_node::<blake2b::Blake2bHasher>(0, &left_key.into(),&value.into() ), hash_base_node::<blake2b::Blake2bHasher>(0, &right_key.into(),&value.into()));
            println!("hash left: {:?}\n hash right: {:?}", branch.left.hash::<blake2b::Blake2bHasher>(), branch.right.hash::<blake2b::Blake2bHasher>());
            println!("merge: {:?}", merge::<blake2b::Blake2bHasher>(height, &key.into(), &branch.left, &branch.right));
        }

        let branch_key = BranchKey::new( height, key.into());
        if let Some(branch) = trie_tree.store().get_branch(&branch_key).unwrap(){
            println!("new_branch[{}]: {:?}", height, branch);
        }

        if let Some(branch) = tree.store().get_branch(&branch_key).unwrap(){
            println!("old_branch[{}]: {:?}", height, branch);
        }
    }


    println!("new_tree: {:?}", trie_tree.store().branches_map());

    println!("root: {:?}, new_root: {:?}", tree.root(), trie_tree.root());

    assert_eq!(tree.root(), trie_tree.root());

}