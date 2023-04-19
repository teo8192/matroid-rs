use std::fmt::Display;
use std::iter::repeat;

use rayon::prelude::*;

use crate::field::Rational;
use crate::matrix::{DynMatrix, Matrix};
use crate::matroid::Matroid;
use crate::set::SetIterator;

use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

pub struct BettiNumbers {
    matrix: DynMatrix<Rational<BigInt>>,
    key: Vec<(usize, (usize, usize))>,
    known_bettis: Vec<(usize, usize, usize)>,
    k: usize,
    n: usize,
}

fn as_rational<I: TryInto<i32>>(n: I) -> Rational<BigInt>
where
    I::Error: std::fmt::Debug,
{
    Rational::from(BigInt::from(n.try_into().unwrap()))
}

/// returns a vec containing (i, j) of interesting betti numbers and a vec where member is count of
/// circuits of cardinality idx. interesting in this sense is that they are non-zero.
/// Uses parallel iterators
#[allow(clippy::type_complexity)]
fn interesting_numbers<M: Matroid + Sync>(matroid: &M) -> (Vec<(usize, usize)>, Vec<usize>) {
    let circuits = matroid.circuits();

    let inums = (2..=(matroid.n() - matroid.k()))
        .flat_map(|i| (0..=matroid.n()).map(move |j| (i, j)))
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(|(i, j)| {
            SetIterator::new(matroid.n())
                .size_limit(*j)
                .equal()
                .filter(|s| matroid.nullity(s) == *i)
                .any(|s| matroid.is_cycle(&s))
        })
        .collect();

    let mut n_vec: Vec<usize> = repeat(0).take(matroid.n() + 1).collect();

    for j in circuits.iter() {
        n_vec[j.size()] += 1;
    }

    (inums, n_vec)
}

impl BettiNumbers {
    pub fn new<M: Matroid + Sync>(matroid: &M) -> Self {
        let n = matroid.n();
        let k = n - matroid.k();
        let (key, circuit_counts) = interesting_numbers(matroid);

        let mut known_bettis = vec![(0, 0, 1)];
        for (j, b) in circuit_counts.iter().enumerate() {
            if *b > 0 {
                known_bettis.push((1, j, *b));
            }
        }

        // the only numbers that will be useful for the equations, are with unique j's (otherwise
        // they have the same coefficient, up to sign)
        let mut seen_j = Vec::new();
        let mut new_key = Vec::new();
        for (i, j) in key.into_iter() {
            if !seen_j.contains(&j) {
                seen_j.push(j);
                new_key.push((i, j));
            } else {
                known_bettis.push((i, j, matroid.betti_number(i, j)));
            }
        }

        // this is to reduce the number of unknowns to our set of equations may solve the rest
        while new_key.len() > k {
            let (i, j) = new_key.remove(0);
            known_bettis.push((i, j, matroid.betti_number(i, j)));
        }

        let key: Vec<(usize, (usize, usize))> = new_key.into_iter().enumerate().collect();

        let matrix = DynMatrix::new(k, key.len() + 1);

        let mut res = BettiNumbers {
            matrix,
            key,
            known_bettis,
            k,
            n,
        }
        .fill_matrix();

        res.matrix.gauss_jordan();

        res
    }

    fn fill_matrix(mut self) -> Self {
        for (idx, (i, j)) in self.key.iter() {
            for s in 0..self.k {
                // coefficient from Herzog-Kuhl equations
                self.matrix[(s, *idx)] =
                    as_rational(-1).exp(*i as i32) * as_rational(*j).exp(s as i32);
            }
        }

        let idx = self.matrix.num_cols() - 1;
        for s in 0..self.k {
            // sets the known constant term
            self.matrix[(s, idx)] = self.constant_term(s as i32);
        }

        self
    }

    fn constant_term(&self, s: i32) -> Rational<BigInt> {
        let mut sum = 0.into();
        for (i, j, betti) in self.known_bettis.iter() {
            // the known term in addition to the coefficient from the Herzog-Kuhl equations
            sum = sum
                + as_rational(-1).exp(*i as i32)
                    * as_rational(*j as i32).exp(s)
                    * as_rational(*betti as i32);
        }

        sum
    }

    /// returns b_{i,j}
    pub fn betti(&self, i: usize, j: usize) -> i32 {
        match (i, j) {
            (0, 0) => 1,
            (0, _) => 0,
            (i, j) => {
                if let Some((t_col, _)) = self.key.iter().find(|(_, (ip, jp))| *ip == i && *jp == j)
                {
                    -self.matrix[(*t_col, self.matrix.num_cols() - 1)]
                        .to_integer()
                        .and_then(|i| i.to_i32())
                        .unwrap()
                } else if let Some((_, _, b)) = self
                    .known_bettis
                    .iter()
                    .find(|(ip, jp, _)| *ip == i && *jp == j)
                {
                    *b as i32
                } else {
                    0
                }
            }
        }
    }

    /// returns list of (i, j, b_{i,j})
    /// b_{i,j} is not in the list if it is zero
    pub fn betti_numbers(&self) -> Vec<(usize, usize, usize)> {
        let mut res = Vec::new();
        for i in 0..=self.k {
            for j in 0..=self.n {
                let betti = self.betti(i, j) as usize;
                if betti != 0 {
                    res.push((i, j, betti));
                }
            }
        }
        res
    }
}

impl Display for BettiNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0 \\leftarrow S / I")?;
        let mut prev = -1;
        for (i, j, betti) in self.betti_numbers() {
            if i as i32 != prev {
                write!(f, " \\leftarrow ")?;
                prev = i as i32;
            } else {
                write!(f, " \\oplus ")?;
            }
            write!(f, "S")?;
            if j != 0 {
                write!(f, "(-{})", j)?;
            }
            if betti != 1 {
                write!(f, "^{{{}}}", betti)?;
            }
        }
        write!(f, "\\leftarrow 0")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matroid::examples::{matroid_1, matroid_2};

    #[test]
    fn from_ex62() {
        // Example 6.2 from the paper "A generalization of weight polynomials to matroids"

        let matroid = matroid_1();

        for n in 0..=(matroid.n() - matroid.k()) {
            for size in 0..=matroid.n() {
                let count = SetIterator::new(matroid.n())
                    .filter(|s| s.size() == size && matroid.nullity(s) == n)
                    .count();
                if count != 0 {
                    println!("nullity {} size {} count {}", n, size, count);
                }
            }
        }

        let betti = BettiNumbers::new(&matroid);

        println!("{}", betti.matrix);

        let betti_nums = vec![
            (0, 0, 1),
            (1, 2, 1),
            (1, 4, 5),
            (2, 5, 4),
            (2, 6, 5),
            (3, 7, 4),
        ];

        assert_eq!(betti.betti_numbers(), betti_nums);
    }

    #[test]
    fn from_ex62_again() {
        let m = matroid_1();
        let n = matroid_2();

        let betti_m = BettiNumbers::new(&m);
        let betti_n = BettiNumbers::new(&n);

        assert_eq!(betti_m.betti_numbers(), betti_n.betti_numbers());
    }
}
