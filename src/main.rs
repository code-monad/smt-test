use std::collections::HashMap;
use std::ptr::hash;
use sparse_merkle_tree::blake2b::Blake2bHasher;
use sparse_merkle_tree::branch::BranchKey;
use {sparse_merkle_tree::*};
use sparse_merkle_tree::merge::hash_base_node;
use sparse_merkle_tree::tree::SparseMerkleTree as OldTree;
use sparse_merkle_tree::trie_tree::SparseMerkleTree as TrieMerkleTree;

use rand::prelude::{Rng, SliceRandom};
#[allow(clippy::upper_case_acronyms)]
type SMT = OldTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
type SMT_NEW = TrieMerkleTree<blake2b::Blake2bHasher, H256, default_store::DefaultStore<H256>>;
use sparse_merkle_tree::merge::MergeValue::{MergeWithZero, ShortCut};
//use sparse_merkle_tree::merge::merge_with_zero;
use sparse_merkle_tree::merge::merge;
use sparse_merkle_tree::traits::StoreReadOps;
use proptest::prelude::*;

fn main() {
    //let mut vec = Vec::new();
    let mut tree = SMT::default();
    let mut trie_tree = SMT_NEW::default();

    let exist_keys =  [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into(),
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into(),
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2].into(),
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3].into(),
        [0, 0, 0, 0, 0, 0, 32, 0, 4, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4].into(),
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5].into(),
        [0, 0, 123, 44, 0, 0, 0, 12, 23, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5].into(),
        [1, 0, 33, 0, 0, 0, 0, 22, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5].into(),
        [1, 0, 33, 0, 0, 0, 0, 22, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5].into(),
        [1, 0, 33, 0, 0, 0, 0, 88, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 24, 3].into(),
        [1, 0, 33, 0, 0, 123, 0, 88, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 24, 3].into(),
        [2, 0, 33, 0, 0, 123, 0, 88, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 24, 3].into(),
    ];

    let non_exist_keys = [
        [0, 0, 0, 0, 2, 245, 222, 236, 217, 227, 7, 222, 6, 57, 141, 228, 126, 83, 116, 156, 7, 244, 144, 6, 142, 155, 104, 75, 87, 158, 109, 80].into(),
        [120, 94, 121, 42, 43, 185, 121, 215, 19, 188, 112, 111, 16, 124, 59, 43, 189, 203, 55, 192, 159, 233, 56, 217, 126, 150, 113, 232, 27, 66, 255, 10].into(),
        [1, 94, 121, 42, 43, 185, 58, 215, 19, 188, 112, 111, 16, 124, 59, 43, 189, 203, 55, 192, 159, 233, 56, 217, 126, 77, 113, 232, 27, 66, 0, 10].into(),
        [24, 8, 121, 2, 43, 7, 1, 215, 9, 188, 112, 4, 16, 124, 59, 8, 6, 67, 33, 32, 24, 5, 56, 217, 126, 150, 113, 232, 27, 66, 255, 10].into(),
        [3, 0, 33, 0, 0, 123, 0, 88, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 24, 3].into(),
        [123, 0, 88, 0, 0, 45, 0, 88, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 24, 3].into(),
    ];

    let exists_keys_len = std::cmp::max(exist_keys.len() / 2, 1);
    let non_exists_keys_len = std::cmp::max(non_exist_keys.len() / 2, 1);

    for key in exist_keys {
        tree.update(key, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into());
        trie_tree.update(key, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into());
    }

    let mut keys: Vec<_> = exist_keys.into_iter().take(exists_keys_len).chain(non_exist_keys.into_iter().take(non_exists_keys_len)).collect();
    keys.dedup();

    let proof = tree.merkle_proof(keys.clone()).expect("gen proof");
    let proof_trie = trie_tree.merkle_proof(keys.clone()).expect("gen proof");

    assert_eq!(proof.leaves_bitmap(), proof_trie.leaves_bitmap());

    assert_eq!(proof.merkle_path(), proof_trie.merkle_path());

    let data: Vec<(H256, H256)> = keys.iter().map(|k|(*k, tree.get(k).expect("get"))).collect();
    let compiled_proof = proof.clone().compile(keys.clone()).expect("compile proof");


    let data_trie: Vec<(H256, H256)> = keys.iter().map(|k|(*k, trie_tree.get(k).expect("get"))).collect();
    let compiled_proof_trie = proof_trie.clone().compile(keys.clone()).expect("compile proof");
    assert_eq!(compiled_proof.0, compiled_proof_trie.0);

    assert!(proof.verify::<blake2b::Blake2bHasher>(tree.root(), data.clone()).expect("verify proof"));
    assert!(compiled_proof.verify::<blake2b::Blake2bHasher>(tree.root(), data.clone()).expect("verify compiled proof"));


    assert!(proof_trie.verify::<blake2b::Blake2bHasher>(trie_tree.root(), data_trie.clone()).expect("verify proof"));
    assert!(compiled_proof_trie.verify::<blake2b::Blake2bHasher>(trie_tree.root(), data_trie.clone()).expect("verify compiled proof"));

    assert_eq!(tree.root(), trie_tree.root());

    test_sub_proof(&compiled_proof_trie, &trie_tree, &data_trie, 20);
}

fn test_sub_proof(
    compiled_proof: &CompiledMerkleProof,
    smt: &SMT_NEW,
    data: &[(H256, H256)],
    test_multi_round: usize,
) {
    let mut keys = data.iter().map(|(k, _v)| *k).collect::<Vec<_>>();

    // test sub proof with single leaf
    for key in &keys {
        let single_compiled_proof = compiled_proof
            .extract_proof::<Blake2bHasher>(data.iter().map(|(k, v)| (*k, *v, k == key)).collect())
            .expect("compiled one proof");
        let expected_compiled_proof = smt
            .merkle_proof(vec![*key])
            .unwrap()
            .compile(vec![*key])
            .unwrap();
        assert_eq!(expected_compiled_proof.0, single_compiled_proof.0);

        let value = smt.get(key).unwrap();
        assert!(single_compiled_proof
            .verify::<Blake2bHasher>(smt.root(), vec![(*key, value)])
            .expect("verify compiled one proof"));
    }

    if data.len() < 2 {
        return;
    }

    // test sub proof with multiple leaves
    let mut rng = rand::thread_rng();
    for _ in 0..test_multi_round {
        keys.shuffle(&mut rng);
        let selected_number = rng.gen_range(2..=keys.len());
        let selected_pairs: HashMap<_, _> = keys
            .iter()
            .take(selected_number)
            .map(|key| (*key, smt.get(key).unwrap()))
            .collect();

        let sub_proof = compiled_proof
            .extract_proof::<Blake2bHasher>(
                data.iter()
                    .map(|(k, v)| (*k, *v, selected_pairs.contains_key(k)))
                    .collect(),
            )
            .expect("compiled sub proof");
        let selected_keys = selected_pairs.keys().cloned().collect::<Vec<_>>();
        let expected_compiled_proof = smt
            .merkle_proof(selected_keys.clone())
            .unwrap()
            .compile(selected_keys)
            .unwrap();
        assert_eq!(expected_compiled_proof.0, sub_proof.0);

        assert!(sub_proof
            .verify::<Blake2bHasher>(smt.root(), selected_pairs.into_iter().collect())
            .expect("verify compiled sub proof"));

    }
}