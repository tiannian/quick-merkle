use digest::{Digest, Output};

use crate::utils;


#[derive(Debug, Clone)]
pub struct Merkle<D: Digest> {
    pub nodes: Vec<Output<D>>,
}

impl<D: Digest> Merkle<D> {
    pub fn new(leafs: Vec<Output<D>>) -> Self {
        // let nodes_len = leafs.len() * 2 - 1;

        let mut nodes = leafs;

        let leafs_rel_end = nodes.len();
        let leafs_pad_end = utils::shift_to_2n(leafs_rel_end);

        nodes.resize(leafs_pad_end, nodes[leafs_rel_end - 1].clone());

        let total_depth = utils::get_n_for_2n(leafs_pad_end);

        let mut pos = 0usize;

        for depth in 0 .. total_depth {
            let iter_count = 1 << (total_depth - 1 - depth);

            for i in 0 .. iter_count {
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

        Self {
            nodes
        }
    }

    // pub fn
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

        for i in 0 .. 6u8 {
            let mut hasher = Sha3_256::new();

            hasher.update(&[i]);

            leafs.push(hasher.finalize());
        }

        let merkle = Merkle::<Sha3_256>::new(leafs);

        for (idx, node) in merkle.nodes.iter().enumerate() {
            let h = hex::encode(node);

            println!("{}: {}", idx, h);
        }

    }

}

