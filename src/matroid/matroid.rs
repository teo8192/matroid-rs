use std::error::Error;
use std::path::Path;

use num_integer::binomial;
use rayon::prelude::*;

use super::storage::StoredMatroid;
use super::{BasesMatroid, CombinatorialDerived, Dual, Elongate};

use crate::betti_nums::BettiNumbers;
use crate::set::{Set, SetIterator};

/// A matroid
///
/// A matriod is something that satisfies one set of the many cryptomorphic sets of axioms for
/// matroid. One such set of axioms is the ones for the rank function.
/// Let r be a rank function. Then
///     * 0 <= r(X) <= |X|
///     * if X is contained in Y, then r(X) <= r(Y)
///     * r(X union Y) + r(X intersection Y) <= r(X) + r(Y)
pub trait Matroid {
    /// The rank of the matroid
    /// Suggest this should be pre-calculated as Self::rank(&Set::of_size(self.n())
    fn k(&self) -> usize;

    /// The size of the ground set
    fn n(&self) -> usize;

    /// the rank of the specified set
    fn rank(&self, subset: &Set) -> usize;

    /// The nullity of the specific subset
    fn nullity(&self, subset: &Set) -> usize {
        if subset.size() == self.n() {
            self.n() - self.k()
        } else {
            subset.size() - self.rank(subset)
        }
    }

    /// calculate the higher hamming distance d_h of the matroid
    fn generalized_hamming_distance(&self, h: usize) -> Option<usize> {
        // the generalized hamming weight is the smallest subset such that the corank of the subset
        // is smaller than or equal the cardinality minus h
        for i in h..=self.n() {
            let subset = SetIterator::new(self.n())
                .size_limit(i)
                .equal()
                .find(|subset| self.corank(subset) <= i - h);

            if subset.is_some() {
                return Some(i);
            }
        }

        None
    }

    /// the corank of the specific subset
    fn corank(&self, subset: &Set) -> usize {
        subset.size() + self.rank(&Set::of_size(self.n()).difference(subset)) - self.k()
    }

    /// checks if a subset is a circuit
    fn is_cycle(&self, subset: &Set) -> bool {
        // circuit cannot be empty
        if subset.is_empty() {
            return false;
        }

        let r = self.rank(subset);

        // if you remove any element, the rank should not change
        (0..self.n()).all(|i| self.rank(&subset.remove_element(i)) == r)
    }

    /// checks if a subset is a circuit
    fn is_circuit(&self, subset: &Set) -> bool {
        // nullity (cardinality - rank) is 1 for circuits
        // and it is a cycle
        subset.size() - self.rank(subset) == 1 && self.is_cycle(subset)
    }

    /// checks if a subset is independent
    fn is_independent(&self, subset: &Set) -> bool {
        self.rank(subset) == subset.size()
    }

    /// Returns a list of all circuits of the matroid
    fn circuits(&self) -> Vec<Set> {
        SetIterator::new(self.n())
            .size_limit(self.k() + 1)
            .smaller_equal()
            .filter(|set| self.is_circuit(set))
            .collect()
    }

    /// Returns a list of all circuits of the matroid, but calculated in parallel
    fn par_circuits(&self) -> Vec<Set>
    where
        Self: Sync,
    {
        let mut circuits = Vec::new();
        for circuit_cardinality in 1..=(self.k() + 1) {
            let circuits_of_cardinality: Vec<Set> = SetIterator::new(self.n())
                .size_limit(circuit_cardinality)
                .equal()
                .par_bridge()
                .filter(|set| self.is_circuit(set))
                .collect();
            circuits.extend(circuits_of_cardinality);
        }
        circuits
    }

    /// Returns a list of all independent sets of the matroid
    fn independents(&self) -> Vec<Set> {
        SetIterator::new(self.n())
            .size_limit(self.k())
            .smaller_equal()
            .filter(|set| self.is_independent(set))
            .collect()
    }

    /// Returns a list of all bases of the matroid
    fn bases(&self) -> Vec<Set> {
        // every base is an independent set of size k
        SetIterator::new(self.n())
            .size_limit(self.k())
            .equal()
            .filter(|set| self.is_independent(set))
            .collect()
    }

    /// the number of bases each element in the ground set is contained in (sorted)
    fn bases_series(&self) -> Vec<usize> {
        let bases = self.bases();
        let mut containment = SetIterator::new(self.n())
            .size_limit(1)
            .equal()
            .map(|element| bases.iter().filter(|&base| &element <= base).count())
            .collect::<Vec<usize>>();
        containment.sort();
        containment
    }

    /// The fundamental circuit of the element e with respect to the basis
    fn fundamental_circuit(&self, e: usize, basis: &Set) -> Option<Set> {
        let c = basis.add_element(e);
        self.circuits()
            .iter()
            .find(|&circuit| circuit <= &c)
            .copied()
    }

    /// Returns a new matroid that is the l'th elongation of self
    fn elongate(&self, l: usize) -> Elongate<Self>
    where
        Self: Sized,
    {
        Elongate::new(self, l)
    }

    /// Returns a new matroid that is the dual of self
    fn dual(&self) -> Dual<Self>
    where
        Self: Sized,
    {
        Dual::from(self)
    }

    /// the combinatorial derived matroid
    fn combinatorial_derived(&self) -> CombinatorialDerived
    where
        Self: Sync + Sized,
    {
        CombinatorialDerived::from_matroid(self)
    }

    /// checks if the matroid is uniform
    /// (i.e. if it has exactly binomial(n, k)=nCk bases)
    /// This will count the number of bases, so it will also generate all the bases, and is a
    /// possibly expensive operation.
    /// Small proof:
    /// If a matroid has nCk bases, then all subsets of size k is a base, and therefore the matroid
    /// has to be uniform.
    fn is_uniform(&self) -> bool {
        self.bases().len() == binomial(self.n(), self.k())
    }

    /// equiality with another matroid
    /// (only checks if they have the same independent sets, not if the matroids are isomorphic)
    fn is_equal<M: Matroid>(&self, other: &M) -> bool {
        if self.n() != other.n() || self.k() != other.k() {
            return false;
        }

        // they must have the same independent and dependent sets
        SetIterator::new(self.n())
            .all(|set| self.is_independent(&set) == other.is_independent(&set))
    }

    /// stores the matroid in a file
    /// automatically adds the extension .matroid to the path
    fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let storage_matroid = StoredMatroid {
            n: self.n(),
            k: self.k(),
            bases: self.bases(),
        };
        storage_matroid.to_file(path)
    }

    /// The betti-numbers of the matroid
    fn betti(&self) -> BettiNumbers
    where
        Self: Sized + Sync,
    {
        BettiNumbers::new(self)
    }

    /// the restriction of self to the set
    fn restrict(&self, element: &Set) -> BasesMatroid {
        let rank = self.rank(element);
        let n = element.size();

        let bases = SetIterator::new(n)
            .size_limit(rank)
            .equal()
            .filter(|s| self.is_independent(&s.extend(element)))
            .collect();

        BasesMatroid::new(bases, n, rank)
    }

    /// The euler characteristic of the matroid
    fn euler_characteristic(&self) -> i32 {
        (0..=self.k())
            .map(|i| {
                SetIterator::new(self.n())
                    .size_limit(i)
                    .equal()
                    .filter(|s| self.is_independent(s))
                    .count() as i32
                    * if i % 2 == 0 { -1 } else { 1 }
            })
            .sum()
    }

    /// The betti number of the given subset
    fn betti_num(&self, sigma: &Set) -> usize {
        if self.is_cycle(sigma) {
            let r = self.rank(sigma);
            self.restrict(sigma).euler_characteristic() * if r % 2 == 0 { -1 } else { 1 }
        } else {
            0
        }
        .try_into()
        .unwrap()
    }

    /// The betti number b_{i,j}
    fn betti_number(&self, i: usize, j: usize) -> usize
    where
        Self: Sync,
    {
        SetIterator::new(self.n())
            .size_limit(j)
            .par_bridge()
            .filter(|s| self.nullity(s) == i)
            .map(|s| self.betti_num(&s))
            .sum()
    }
}

/// Load a matroid from a file
/// automatically adds the extension .matroid to the path
#[allow(unused)]
pub fn load_matroid(path: &Path) -> Result<BasesMatroid, Box<dyn Error>> {
    let storage_matroid = StoredMatroid::from_file(path)?;
    Ok(storage_matroid.into())
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::MatrixMatroid;

    use crate::matrix::DynMatrix;
    use crate::matroid::UniformMatroid;

    use tinyfield::prime_field::PrimeField;
    use tinyfield::GF2;

    use std::collections::HashMap;
    use std::env::temp_dir;
    use uuid::Uuid;
    #[test]
    fn equiality() {
        let one = GF2::one;

        let umatrix = DynMatrix::from_rows(&[&[one, one, one, one]]).unwrap();
        let uniform_from_matrix = MatrixMatroid::from(umatrix);

        let u14 = UniformMatroid::new(1, 4);

        assert!(u14.is_equal(&uniform_from_matrix));
    }

    #[test]
    fn storage() {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        let matroid = UniformMatroid::new(3, 6);

        matroid.save(&path).unwrap();

        let loaded = load_matroid(&path).unwrap();

        let original_independents = matroid.independents();
        let loaded_independents = loaded.independents();

        assert_eq!(original_independents, loaded_independents);
    }

    #[test]
    fn uniformity() {
        let u37 = UniformMatroid::new(3, 7);
        let m = crate::matroid::examples::matroid_1();

        assert!(u37.is_uniform());
        assert!(!m.is_uniform());
    }

    #[test]
    fn restrict() {
        let u36 = UniformMatroid::new(3, 6);
        let u34 = UniformMatroid::new(3, 4);
        let u33 = UniformMatroid::new(3, 3);
        let u22 = UniformMatroid::new(2, 2);

        let restricted = u36.restrict(&0b111010.into());
        assert!(restricted.is_equal(&u34));

        let restricted = u36.restrict(&0b101001.into());
        assert!(restricted.is_equal(&u33));

        let restricted = u36.restrict(&0b001010.into());
        assert!(restricted.is_equal(&u22));
    }

    #[test]
    fn betti_nums() {
        let u36 = UniformMatroid::new(3, 6);

        for circuit in u36.circuits() {
            assert_eq!(u36.betti_num(&circuit), 1);
        }

        for i in SetIterator::new(u36.n()) {
            if u36.is_cycle(&i) {
                assert!(u36.betti_num(&i) > 0);
            } else {
                assert_eq!(u36.betti_num(&i), 0);
            }
        }
    }

    #[test]
    fn betti_nums_circuits() {
        let du36 = UniformMatroid::new(3, 6).combinatorial_derived();

        for circuit in du36.circuits() {
            assert_eq!(du36.betti_num(&circuit), 1);
        }
    }

    #[test]
    fn betti_nums2() {
        let u25 = UniformMatroid::new(2, 5).combinatorial_derived();
        // let u25 = matroid_1();

        let mut hm = HashMap::<(usize, usize), usize>::new();

        for s in SetIterator::new(u25.n()) {
            let i = u25.nullity(&s);
            let j = s.size();
            let betti = u25.betti_num(&s);
            if betti == 0 {
                continue;
            }
            if let Some(n) = hm.get_mut(&(i, j)) {
                *n += betti;
            } else {
                hm.insert((i, j), betti);
            }
        }

        let mut v = vec![(0, 0, 1)];
        for ((i, j), n) in hm {
            v.push((i, j, n));
        }
        v.sort();

        assert_eq!(v, u25.betti().betti_numbers());
    }

    #[test]
    fn corank() {
        let matroid = UniformMatroid::new(3, 7);
        let set = 0b1110100.into();

        assert_eq!(matroid.corank(&set), 4);
    }

    #[test]
    fn generalized_hamming_distance() {
        let matroid = UniformMatroid::new(3, 7);

        assert_eq!(matroid.generalized_hamming_distance(1), Some(5));
        assert_eq!(matroid.generalized_hamming_distance(2), Some(6));
        assert_eq!(matroid.generalized_hamming_distance(3), Some(7));
        assert_eq!(matroid.generalized_hamming_distance(4), None);
    }
}
