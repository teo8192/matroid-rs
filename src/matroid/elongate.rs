use crate::set::Set;

use super::Matroid;

/// the elongation of a matroid
/// see section 2.5 in <https://doi.org/10.1016/j.disc.2015.10.005>
pub struct Elongate<'a, M: Matroid> {
    matroid: &'a M,
    elongation: usize,
}

impl<'a, M: Matroid> Elongate<'a, M> {
    /// create elongation of matroid
    pub fn new(matroid: &'a M, elongation: usize) -> Self {
        Elongate { matroid, elongation }
    }
}

impl<'a, M: Matroid> Matroid for Elongate<'a, M> {
    fn rank(&self, subset: &Set) -> usize {
        // the elongation of a matroid
        let r = self.matroid.rank(subset);
        let nullity = subset.size() - r;
        if nullity > self.elongation {
            r + self.elongation
        } else {
            subset.size()
        }
    }

    fn k(&self) -> usize {
        self.matroid.k() + self.elongation
    }

    fn n(&self) -> usize {
        self.matroid.n()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::MatrixMatroid;

    use crate::matrix::DynMatrix;

    use tinyfield::prime_field::PrimeField;
    use tinyfield::GF2;

    #[test]
    fn elongation() {
        let one = GF2::one;
        let zero = GF2::zero;
        let a = DynMatrix::from_rows(&[
            &[zero, zero, zero, one, one, one, one],
            &[zero, one, one, zero, zero, one, one],
            &[one, zero, one, zero, one, zero, one],
        ])
        .unwrap();

        let matroid = MatrixMatroid::from(a);

        println!("{:?}", matroid.circuits());

        let elongated = matroid.elongate(1);

        println!("{:?}", elongated.circuits());

        let elongated = matroid.elongate(2);

        println!("{:?}", elongated.circuits());
    }

    #[test]
    fn small_elongation() {
        let one = GF2::one;
        let zero = GF2::zero;
        let a = DynMatrix::from_rows(&[&[one, zero, one, one], &[zero, one, one, zero]]).unwrap();

        let matroid = MatrixMatroid::from(a);
        println!("{:?}", matroid.circuits());
        let elongated = matroid.elongate(1);
        println!("{:?}", elongated.circuits());
    }
}
