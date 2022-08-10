use digest::{Digest, Output};

use crate::{utils, MerkleProof, ProofNode};

#[derive(Debug, Clone)]
pub struct Merkle<D: Digest> {
    pub nodes: Vec<Output<D>>,
    pub depth: u32,
}

impl<D: Digest> Merkle<D> {
    pub fn new(leafs: Vec<Output<D>>) -> Option<Self> {
        let mut nodes = leafs;

        let leafs_rel_end = nodes.len();

        if leafs_rel_end == 0 {
            return None;
        }

        if leafs_rel_end == 1 {
            return Some(Merkle { nodes, depth: 1 });
        }

        let leafs_pad_end = utils::shift_to_2n(leafs_rel_end);

        let nodes_len = leafs_rel_end * 2 - 1;

        nodes.reserve(nodes_len);

        nodes.resize(leafs_pad_end, nodes[leafs_rel_end - 1].clone());

        let total_depth = utils::get_n_for_2n(leafs_pad_end);

        let mut pos = 0usize;

        for depth in 0..total_depth {
            let iter_count = 1 << (total_depth - 1 - depth);

            for i in 0..iter_count {
                let mut hasher = D::new();

                let first_idx = pos;
                let second_idx = pos + 1;

                log::debug!("depth: {}, iter: {}, hash idx: {}", depth, i, first_idx);

                hasher.update(&nodes[first_idx]);
                hasher.update(&nodes[second_idx]);

                let hash = hasher.finalize();

                nodes.push(hash);

                pos = first_idx + 2;
            }
        }

        Some(Self {
            nodes,
            depth: total_depth as u32,
        })
    }

    pub fn root(&self) -> &Output<D> {
        self.nodes.last().expect("This unwrap will never throw.")
    }

    pub fn build_proof(&self, idx: usize) -> MerkleProof<D> {
        let mut begin = 0;
        let mut rel_offset = idx;

        let mut nodes = Vec::with_capacity(self.depth as usize);

        for i in 0..self.depth {
            let offset = if rel_offset & 1 == 0 {
                let offset = rel_offset + 1;
                rel_offset /= 2;
                offset
            } else {
                let offset = rel_offset - 1;
                rel_offset = offset / 2;
                offset
            };

            let idx = begin + offset;

            log::debug!("begin: {}, rel_offset: {}, offset: {}, idx: {}", begin, rel_offset, offset, idx);

            let node = self.nodes[idx].clone();

            nodes.push(ProofNode::new(idx, node));

            begin += 1 << (self.depth - i);
        }

        MerkleProof {
            leaf: self.nodes[idx].clone(),
            nodes,
            root: self.root().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use digest::Digest;
    use sha3::Sha3_256;

    use crate::Merkle;

    #[test]
    fn test_merkle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut leafs = Vec::with_capacity(6);

        for i in 0..9u8 {
            let mut hasher = Sha3_256::new();

            hasher.update(&[i]);

            leafs.push(hasher.finalize());
        }

        let merkle = Merkle::<Sha3_256>::new(leafs).unwrap();

        for (idx, node) in merkle.nodes.iter().enumerate() {
            let h = hex::encode(node);

            println!("{}: {}", idx, h);
        }

        let proof = merkle.build_proof(5);

        assert!(proof.verify());
    }
}
