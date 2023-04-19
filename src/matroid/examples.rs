use crate::set::{Set, SetIterator};

use super::BasesMatroid;

/// This is the matroid M from exampe 6.2 in the paper "A generalization of weight polynomials to matroids"
/// <https://doi.org/10.1016/j.disc.2015.10.005>
/// This has the same betty numbers as another non-isomorphic matroid (see matroid_2)
#[allow(unused)]
pub fn matroid_1() -> BasesMatroid {
    let mut bases: Vec<Set> = vec![
        vec![1, 3, 4, 6, 7],
        vec![1, 2, 3, 6, 8],
        vec![1, 2, 3, 4, 8],
        vec![1, 2, 3, 5, 8],
        vec![1, 2, 5, 6, 8],
        vec![1, 2, 3, 4, 7],
        vec![1, 2, 3, 5, 7],
        vec![1, 2, 5, 6, 7],
        vec![1, 3, 4, 5, 7],
        vec![1, 3, 4, 6, 8],
        vec![1, 2, 4, 6, 8],
        vec![1, 2, 4, 6, 7],
        vec![1, 3, 4, 5, 8],
        vec![1, 2, 4, 5, 7],
        vec![1, 4, 5, 6, 7],
        vec![1, 2, 3, 6, 7],
        vec![1, 3, 5, 6, 7],
        vec![1, 4, 5, 6, 8],
        vec![1, 3, 5, 6, 8],
        vec![1, 2, 4, 5, 8],
    ]
    .into_iter()
    .map(|mut v: Vec<usize>| {
        v.iter_mut().for_each(|x| *x -= 1);
        v.into()
    })
    .collect();

    debug_assert_eq!(bases.len(), 20);

    BasesMatroid::new(bases, 8, 5)
}

/// This is the matroid N from example 6.2 in the paper "A generalization of weight polynomials to matroids"
/// <https://doi.org/10.1016/j.disc.2015.10.005>
/// This has the same betty numbers as another non-isomorphic matroid (see matroid_1)
#[allow(unused)]
pub fn matroid_2() -> BasesMatroid {
    let mut bases: Vec<Set> = vec![
        vec![1, 3, 4, 6, 7],
        vec![1, 2, 3, 4, 8],
        vec![1, 2, 3, 5, 8],
        vec![1, 2, 5, 6, 8],
        vec![1, 2, 3, 4, 7],
        vec![1, 2, 3, 5, 7],
        vec![1, 2, 5, 6, 7],
        vec![1, 3, 4, 5, 7],
        vec![1, 3, 4, 6, 8],
        vec![1, 2, 4, 6, 8],
        vec![1, 2, 4, 6, 7],
        vec![1, 3, 4, 5, 8],
        vec![1, 2, 4, 5, 7],
        vec![1, 3, 4, 5, 6],
        vec![1, 2, 4, 5, 6],
        vec![1, 3, 5, 6, 7],
        vec![1, 2, 3, 5, 6],
        vec![1, 2, 3, 4, 6],
        vec![1, 3, 5, 6, 8],
        vec![1, 2, 4, 5, 8],
    ]
    .into_iter()
    .map(|mut v: Vec<usize>| {
        v.iter_mut().for_each(|x| *x -= 1);
        v.into()
    })
    .collect();

    debug_assert_eq!(bases.len(), 20);

    BasesMatroid::new(bases, 8, 5)
}

/// This example is an example of a non-fast graphical matroid of nullity 4 and rank 2.
/// it is the matroid of a graph on 3 vertices, with two edged between any pair of vertices
#[allow(unused)]
pub fn non_fast_matroid() -> BasesMatroid {
    let bases: Vec<Set> = SetIterator::new(6)
        .size_limit(2)
        .equal()
        .filter(|set| {
            !(0..3).any(|i| {
                // this is the parallel edges
                let circuit: Set = (3 << (i * 2)).into();
                circuit <= *set
            })
        })
        .collect();

    BasesMatroid::new(bases, 6, 2)
}

#[cfg(test)]
mod tests {
    use crate::matroid::Matroid;

    use super::*;

    #[test]
    fn non_equal_matroids() {
        let m1 = matroid_1();
        let m2 = matroid_2();

        assert!(!m1.is_equal(&m2));
        assert_ne!(m1.bases_series(), m2.bases_series());
    }

    #[test]
    fn non_equal_derived() {
        let m1 = matroid_1().combinatorial_derived();
        let m2 = matroid_2().combinatorial_derived();

        assert!(!m1.is_equal(&m2));
        assert_ne!(m1.bases_series(), m2.bases_series());
    }
}
