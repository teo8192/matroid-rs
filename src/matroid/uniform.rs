use crate::matroid::Matroid;
use crate::set::Set;

#[allow(unused_macros)]
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

#[allow(unused_macros)]
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

/// The U(K, N) uniform matroid.
/// This is on a ground set of N elements, where every subset of K or fewer elements are independent.
#[derive(Debug, Clone)]
pub struct UniformMatroid {
    k: usize,
    n: usize,
}

impl UniformMatroid {
    pub fn new(k: usize, n: usize) -> Self {
        UniformMatroid { k, n }
    }
}

impl Matroid for UniformMatroid {
    fn rank(&self, subset: &Set) -> usize {
        min!(subset.size(), self.k)
    }

    fn k(&self) -> usize {
        self.k
    }

    fn n(&self) -> usize {
        self.n
    }

    fn is_uniform(&self) -> bool {
        true
    }

    fn combinatorial_derived(&self) -> super::CombinatorialDerived
    where
        Self: Sync + Sized,
    {
        super::CombinatorialDerived::from_matroid(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u14() {
        let matroid = UniformMatroid::new(1, 4);

        // number of bases should be 4 choose 1
        assert_eq!(matroid.bases().len(), 4);

        // number of circuits should be 4 choose 2
        assert_eq!(matroid.circuits().len(), 6);
    }

    #[test]
    fn u36() {
        let matroid = UniformMatroid::new(3, 6);

        // number of bases should be 6 choose 3
        assert_eq!(matroid.bases().len(), 20);

        // number of circuits should be 6 choose 4
        assert_eq!(matroid.circuits().len(), 15);
    }
}
