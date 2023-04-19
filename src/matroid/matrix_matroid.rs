use crate::matrix::{DynMatrix, Matrix};
use crate::set::Set;

use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Matroid;

#[derive(Debug)]
pub struct MatrixMatroid<E>
where
    E: Copy
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq,
{
    matrix: DynMatrix<E>,
    rank: usize,
}

impl<E> Matroid for MatrixMatroid<E>
where
    E: Copy
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq,
{
    fn rank(&self, subset: &Set) -> usize {
        let v: Vec<usize> = subset.into();
        let mut a = self.matrix.subset_matrix(&v);
        a.gauss_jordan();
        a.rank()
    }

    fn k(&self) -> usize {
        self.rank
    }

    fn n(&self) -> usize {
        self.matrix.num_cols()
    }
}

impl<E> From<DynMatrix<E>> for MatrixMatroid<E>
where
    E: Copy
        + Add<Output = E>
        + Sub<Output = E>
        + Mul<Output = E>
        + Div<Output = E>
        + Neg<Output = E>
        + From<u8>
        + PartialEq,
{
    fn from(mut matrix: DynMatrix<E>) -> Self {
        matrix.gauss_jordan();
        MatrixMatroid {
            rank: matrix.rank(),
            matrix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tinyfield::prime_field::PrimeField;
    use tinyfield::GF2;

    #[test]
    fn matrix_matroid() {
        let one = GF2::one;
        let zero = GF2::zero;
        let a = DynMatrix::from_rows(&[&[one, zero, one, one], &[zero, one, one, zero]]).unwrap();

        let matroid = MatrixMatroid::from(a);

        assert!(matroid.rank(&[0usize, 3].into()) == 1);
        assert!(matroid.rank(&[0usize, 1].into()) == 2);
    }
}
