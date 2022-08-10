use digest::{Digest, Output};

use crate::utils;

#[derive(Debug, Clone)]
pub enum ProofNode<D: Digest> {
    Left(Output<D>),
    Right(Output<D>),
}

impl<D: Digest> ProofNode<D> {
    pub fn new(idx: usize, node: Output<D>) -> Self {
        if idx & 1 == 0 {
            Self::Left(node)
        } else {
            Self::Right(node)
        }
    }
}

#[derive(Debug, Clone)]
pub struct MerkleProof<D: Digest> {
    pub root: Output<D>,
    pub nodes: Vec<ProofNode<D>>,
    pub leaf: Output<D>,
}

impl<D: Digest> MerkleProof<D> {
    pub fn verify(&self) -> bool {
        let mut leaf = self.leaf.clone();

        for node in &self.nodes {
            leaf = match node {
                ProofNode::Left(n) => utils::hash_2_node::<D>(n.clone(), leaf),
                ProofNode::Right(n) => utils::hash_2_node::<D>(leaf, n.clone()),
            }
        }

        self.root == leaf
    }
}
