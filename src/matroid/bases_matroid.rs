use crate::set::Set;

use super::Matroid;

#[derive(Debug)]
pub struct BasesMatroid {
    n: usize,
    k: usize,
    bases: Vec<Set>,
}

impl BasesMatroid {
    #[allow(unused)]
    /// Create a matroid from a list of bases.
    /// Every base should have k elements (the rank of the matroid), and the cadrinality of the
    /// ground set is n.
    pub fn new(bases: Vec<Set>, n: usize, k: usize) -> Self {
        debug_assert!(k <= n);
        debug_assert!(bases.iter().all(|&x| x.size() == k));
        Self { bases, n, k }
    }

    /// calculate the rank of a subset given a list of bases
    /// It is assumed that all the bases are the same size
    pub fn rank_of_subset_given_bases(subset: &Set, bases: &[Set]) -> usize {
        let mut max = 0;
        for base in bases {
            let intersect_size = base.intersect(subset).size();
            if intersect_size > max {
                max = intersect_size;
            }
            // if the max is already the rank, then we can stop
            if max == base.size() {
                break;
            }
        }

        max
    }
}

impl Matroid for BasesMatroid {
    fn n(&self) -> usize {
        self.n
    }

    fn k(&self) -> usize {
        self.k
    }

    fn rank(&self, subset: &Set) -> usize {
        Self::rank_of_subset_given_bases(subset, &self.bases)
    }
}
