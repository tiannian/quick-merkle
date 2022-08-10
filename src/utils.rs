use digest::{Digest, Output};

pub fn shift_to_2n(n: usize) -> usize {
    (n & (n - 1)) << 1
}

pub fn bit(n: usize, pos: u32) -> bool {
    n & 1 << pos != 0
}

pub fn get_n_for_2n(n: usize) -> usize {
    for i in 0..usize::BITS {
        if bit(n, i) {
            return i as usize;
        }
    }

    return 0;
}

pub fn hash_2_node<D: Digest>(x: Output<D>, y: Output<D>) -> Output<D> {
    let mut hasher = D::new();

    hasher.update(&x);
    hasher.update(&y);

    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use crate::utils::shift_to_2n;

    use super::get_n_for_2n;

    #[test]
    fn test_n_for_2n() {
        assert_eq!(get_n_for_2n(5), 0);
        assert_eq!(get_n_for_2n(4), 2);
        assert_eq!(get_n_for_2n(0), 0);
    }

    #[test]
    fn quick_shift() {
        assert_eq!(16, shift_to_2n(9));
    }
}
