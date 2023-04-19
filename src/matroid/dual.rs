use std::fmt::{Debug, Formatter};

use crate::set::Set;

use super::Matroid;

/// The dual matroid of a matroid
pub struct Dual<'a, M: Matroid> {
    matroid: &'a M,
}

impl<'a, M: Matroid> Matroid for Dual<'a, M> {
    fn rank(&self, subset: &Set) -> usize {
        self.matroid
            .rank(&Set::of_size(self.matroid.n()).difference(subset))
            + subset.size()
            - self.matroid.k()
    }

    fn n(&self) -> usize {
        self.matroid.n()
    }

    fn k(&self) -> usize {
        self.matroid.n() - self.matroid.k()
    }
}

impl<'a, M: Matroid> From<&'a M> for Dual<'a, M> {
    fn from(matroid: &'a M) -> Self {
        Self { matroid }
    }
}

impl<'a, M: Matroid + Debug> Debug for Dual<'a, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dual")
            .field("matroid", &self.matroid)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matrix::DynMatrix;
    use crate::matroid::{MatrixMatroid, UniformMatroid};

    use tinyfield::prime_field::PrimeField;
    use tinyfield::GF2;

    #[test]
    fn double_dual() {
        let matroid = UniformMatroid::new(2, 6);
        let dual = Dual::from(&matroid);
        let double_dual = Dual::from(&dual);

        assert!(matroid.is_equal(&double_dual));
    }

    #[test]
    fn u26dual() {
        let matroid = UniformMatroid::new(2, 6);
        let dual = Dual::from(&matroid);

        let u46 = UniformMatroid::new(4, 6);

        assert!(dual.is_equal(&u46));
    }

    #[test]
    fn hamming_code() {
        let one = GF2::one;
        let zer = GF2::zero;

        let g = DynMatrix::from_rows(&[
            &[one, zer, zer, zer, zer, one, one],
            &[zer, one, zer, zer, one, zer, one],
            &[zer, zer, one, zer, one, one, zer],
            &[zer, zer, zer, one, one, one, one],
        ])
        .unwrap();
        let h = DynMatrix::from_rows(&[
            &[zer, one, one, one, one, zer, zer],
            &[one, zer, one, one, zer, one, zer],
            &[one, one, zer, one, zer, zer, one],
        ])
        .unwrap();

        let matroid = MatrixMatroid::from(g);
        let dual = Dual::from(&matroid);
        let matroid_of_dual = MatrixMatroid::from(h);

        assert!(dual.is_equal(&matroid_of_dual));
    }
}
