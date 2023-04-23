use matroids::set::{Set, SetIterator};

use matroids::matrix::{DynMatrix};
use matroids::matroid::{MatrixMatroid, Matroid};

use tinyfield::prime_field::PrimeField;
use tinyfield::GF2;

macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

#[allow(unused)]
fn vector_repr(a: &Set) -> String {
    let mut repr = String::new();
    for i in (0..7).rev() {
        if !repr.is_empty() {
            repr.push_str(", ");
        }
        if a.contains_element(i) {
            repr.push('1');
        } else {
            repr.push('0');
        }
    }
    repr
}

fn support(sets: &[Set]) -> Set {
    Set::of_size(sets.len()).union_of_sets(sets)
}

/// calculate the distances from the generators
fn distances_from_generators(generators: &[Set]) -> Vec<usize> {
    let elements: Vec<_> = SetIterator::new(generators.len())
        .filter(|x| !x.is_empty())
        .map(|set| set.kirkhoff_sum(generators))
        .collect();

    let mut w = vec![usize::MAX; generators.len()];
    let mut n_subspace = vec![0; generators.len()];

    for i in 1..=(generators.len()) {
        'subspace_selector: for set in SetIterator::new(elements.len()).size_limit(i).equal() {
            let mut subspace = Vec::new();
            for subset in SetIterator::new(i) {
                let res = subset.extend(&set).kirkhoff_sum(&elements);

                // check for dependencies among the generators (should be none, but skip subspace
                // if there are dependencies)
                if res.size() == 0 && subset.size() != 0 {
                    continue 'subspace_selector;
                }

                subspace.push(subset.extend(&set).kirkhoff_sum(&elements));
            }

            w[i - 1] = min!(w[i - 1], support(&subspace).size());
            n_subspace[i - 1] += 1;
        }
    }

    w
}

fn main() {
    let generators = vec![
        Set::from(0b1000011),
        Set::from(0b0100101),
        Set::from(0b0010110),
        Set::from(0b0001111),
    ];

    let w = distances_from_generators(&generators);
    println!("w = {:?}", w);

    let one = GF2::one;
    let zer = GF2::zero;
    let matrix = DynMatrix::from_rows(&[
        &[one, zer, zer, zer, zer, one, one],
        &[zer, one, zer, zer, one, zer, one],
        &[zer, zer, one, zer, one, one, zer],
        &[zer, zer, zer, one, one, one, one],
    ]).unwrap();

    let matroid = MatrixMatroid::from(matrix);
    for i in 1..=matroid.k() {
        println!("{}: {}", i, matroid.generalized_hamming_distance(i).unwrap());
    }

    let generators = vec![
        Set::from(0b0001111),
        Set::from(0b0110011),
        Set::from(0b1010101),
    ];

    let w = distances_from_generators(&generators);
    println!("w = {:?}", w);

    let dual = matroid.dual();
    for i in 1..=dual.k() {
        println!("{}: {}", i, dual.generalized_hamming_distance(i).unwrap());
    }
}
