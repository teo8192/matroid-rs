#[cfg(feature = "progress")]
use std::sync::atomic::AtomicUsize;

use super::{BasesMatroid, Matroid};

use rayon::prelude::*;

use crate::set::{Set, SetIterator};

use dashmap::DashSet;

use log::info;

#[cfg(feature = "progress")]
use indicatif::ProgressBar;

#[cfg(feature = "progress")]
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

#[cfg(feature = "progress")]
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

/// Do the epsilon operation on the circuits
fn epsilon(dependents: &[Set], rank: usize) -> Vec<Set> {
    let dependent = DashSet::new();

    // the next variables are to do with progress reporting
    #[cfg(feature = "progress")]
    let progress = {
        // the number of iterations is the len - 2 + len - 3 + ... + 1
        // we know that n + (n - 1) + (n - 2) + ... + 1 = n * (n + 1) / 2
        let len = dependents.len() - 2;
        let total_iterations = if len & 1 == 0 {
            (len + 1) * (len / 2)
        } else {
            len * ((len + 1) / 2)
        };

        ProgressBar::new(total_iterations as u64)
    };

    (0..(dependents.len() - 1)).par_bridge().for_each(|i| {
        dependent.insert(dependents[i]);
        for j in (i + 1)..dependents.len() {
            let intersect = dependents[i].intersect(&dependents[j]);
            if dependents[i].size() + dependents[j].size() - intersect.size() - 1 > rank {
                #[cfg(feature = "progress")]
                progress.inc(1);

                continue;
            }
            // the intersection has to be not contained in dependents already.
            // we already know that no set in dependents has cardinality 1 or 2,
            // so if the intersect has this cardinality, then it is not in the set.
            // Otherwise, we need to check through all the sets in dependents
            // the case when intersect is 0, the if test will be false
            if (intersect.size() < 3 && intersect.size() > 0)
                || (intersect.size() >= 3 && !dependents.iter().any(|b| b <= &intersect))
            {
                let upper = intersect.size();
                for count in 0..upper {
                    let elem = Set::from(1 << count).extend(&intersect);
                    let set = dependents[i].union(&dependents[j]).difference(&elem);
                    // this might be a redundant if test
                    // (size should be equal to di + dj - intersect - 1)
                    if set.size() <= rank {
                        dependent.insert(set);
                    }
                }
            }

            #[cfg(feature = "progress")]
            progress.inc(1);
        }
    });

    #[cfg(feature = "progress")]
    progress.finish();

    dependent.insert(dependents[dependents.len() - 1]);

    dependent.into_iter().collect()
}

/// Find all bases with respect to a set of dependent sets
/// The dependent set could either be all dependents, or just the circuits
fn bases_from_dependents(dependents: &[Set], num_points: usize, rank: usize) -> Vec<Set> {
    SetIterator::new(num_points)
        .size_limit(rank)
        .equal()
        .par_bridge()
        .filter(|subset| {
            // the subset cannot contain a dependent set
            !dependents.iter().any(|dependent| dependent <= subset)
        })
        .collect()
}

/// Find the initial dependents, but with a limit of the cardinality of the support
/// points should be a list of circuits in the original matroid
fn initial_dependents_support_limit<M: Matroid + Sync>(
    matroid: &M,
    points: &[Set],
    upper_derived_rank: usize,
) -> Vec<Set> {
    #[cfg(feature = "progress")]
    let max: usize = max!(1usize << (points.len() - min!(10, points.len())), 1);
    #[cfg(feature = "progress")]
    let status = AtomicUsize::new(0);
    #[cfg(feature = "progress")]
    let progress = ProgressBar::new(1024);

    let mut res = Vec::new();

    for subset_size in 3..=upper_derived_rank {
        // add all subsets with cardinality larger than nullity of the union of the circuits of the
        // given cardinality
        let vec: Vec<Set> = SetIterator::new(points.len())
            .size_limit(subset_size)
            .equal()
            .par_bridge()
            .filter(|subset| {
                #[cfg(feature = "progress")]
                {
                    let curr_stat = usize::from(subset) / max;
                    let prev_stat = status.swap(curr_stat, std::sync::atomic::Ordering::Relaxed);
                    if curr_stat > prev_stat {
                        progress.inc(1);
                    }
                }
                let circuit_union = points
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| subset.contains_element(*i))
                    .fold(Set::empty(), |acc, (_, c)| acc.union(c));

                subset.size() > matroid.nullity(&circuit_union)
            })
            .collect();
        res.extend(vec);
    }

    #[cfg(feature = "progress")]
    {
        progress.finish();
    }

    res
}

/// find the inclusion minimal elements
fn inclusion_minimal(subsets: &[Set]) -> Vec<Set> {
    subsets
        .into_par_iter()
        .filter(|subset| {
            // if subset is inclusion minimal, it does not contain any other subset
            subset.size() == 3 || !subsets.iter().any(|b| b < subset)
        })
        .cloned()
        .collect()
}

#[derive(Debug)]
pub struct CombinatorialDerived {
    rank: usize,
    elements: Vec<Set>,
    bases: Vec<Set>,
}

impl CombinatorialDerived {
    /// Calculate the combinatorial derived matroid from a matroid.
    pub fn from_matroid<M: Matroid + Sync>(matroid: &M) -> Self {
        if matroid.is_uniform() || matroid.n() <= 3 {
            Self::from_fast_matroid(matroid)
        } else {
            Self::from_non_fast_matroid(matroid)
        }
    }

    /// Calculate the combinatorial derived matroid from a fast matroid.
    /// examples of fast matroids are uniform matroids and matroids with n <= 3
    fn from_fast_matroid<M: Matroid + Sync>(matroid: &M) -> Self {
        let rank = matroid.n() - matroid.k();

        let elements = matroid.circuits();

        let bases = SetIterator::new(elements.len())
            .size_limit(rank)
            .equal()
            .par_bridge()
            .filter(|set| {
                // the subset cannot contain a dependent set
                SetIterator::new(set.size())
                    .size_limit(3)
                    .greater_equal()
                    .all(|subset| {
                        // this extend operation will make the extended be the subset selected by
                        // subset of set
                        let extended = subset.extend(set);
                        matroid.nullity(&extended.union_of_sets(&elements)) >= extended.size()
                    })
            })
            .collect::<Vec<_>>();

        Self {
            rank,
            elements,
            bases,
        }
    }

    /// Caclulate the combinatorial derived matroid from a non-fast matroid
    fn from_non_fast_matroid<M: Matroid + Sync>(matroid: &M) -> Self {
        let mut rank = matroid.n() - matroid.k();

        let elements = matroid.circuits();

        info!("Calculating initial dependents...");
        let mut dependents = initial_dependents_support_limit(matroid, &elements, rank);
        info!("Finding inclusion minimal...");
        dependents = inclusion_minimal(&dependents);

        let mut cardinality = dependents.len();
        info!("First cardinality of dependents: {}", cardinality);

        loop {
            info!("Doing epsilon...");
            dependents = epsilon(&dependents, rank);
            info!("Finding inclusion minimal...");
            dependents = inclusion_minimal(&dependents);
            info!("Cardinality of dependents: {}", dependents.len());
            if dependents.len() == cardinality {
                break;
            }

            cardinality = dependents.len();
        }

        info!("Finding bases...");
        let mut bases = bases_from_dependents(&dependents, elements.len(), rank);

        // bases are empty if every set of size rank is dependent
        while bases.is_empty() {
            info!("Decreasing rank of the combinatorial derived matroid!");
            if rank == 0 {
                // this should be impossible, since it is proved that the matroid is simple (no
                // dependent of size 1 or 2).
                // the only case I can think of where this might happen is if the matroid is U_0,1,
                // but this is a fast matroid so this function should never run in that case.
                panic!("got negative rank for the combinatorial derived matroid");
            }
            rank -= 1;
            bases = bases_from_dependents(&dependents, elements.len(), rank);
        }

        info!(
            "Done calculating combinatorial derived matroid, {} bases, rank: {} on {} elements!",
            bases.len(),
            rank,
            elements.len()
        );

        Self {
            rank,
            elements,
            bases,
        }
    }

    /// returns the union of all circuits in the subset
    pub fn circuit_union(&self, subset: &Set) -> Set {
        subset.union_of_sets(&self.elements)
    }

    /// checks if the subset is completely redundant
    pub fn completly_redundant(&self, subset: &Set) -> bool {
        let s = self.circuit_union(subset);
        (0..subset.size())
            .filter(|e| subset.contains_element(*e))
            .all(|e| self.circuit_union(&subset.remove_element(e)) == s)
    }
}

impl<M: Matroid + Sync> From<&M> for CombinatorialDerived {
    fn from(matroid: &M) -> Self {
        CombinatorialDerived::from_matroid(matroid)
    }
}

impl Matroid for CombinatorialDerived {
    fn rank(&self, subset: &Set) -> usize {
        // this matroid is simple, so if the subset has size less than 3, then the rank is the size
        if subset.size() < 3 {
            return subset.size();
        }

        // are calculated the same way as the bases matroid
        BasesMatroid::rank_of_subset_given_bases(subset, &self.bases)
    }

    fn k(&self) -> usize {
        self.rank
    }

    fn n(&self) -> usize {
        self.elements.len()
    }

    fn bases(&self) -> Vec<Set> {
        self.bases.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{matroid::UniformMatroid, utils::contains_same_elems};

    #[test]
    fn uniform_3_6() {
        let matroid = UniformMatroid::new(3, 6);

        let derived = CombinatorialDerived::from(&matroid);

        let circuits = derived.circuits();

        // the following tests are from the table in the paper
        // https://arxiv.org/abs/2206.06881

        for i in 0..10 {
            println!(
                "size {}: {}",
                i,
                circuits.iter().filter(|x| x.size() == i).count()
            );
        }

        // should be no circuits smaller than 3
        assert!(circuits.iter().all(|x| x.size() >= 3));

        // should be no circuits greater than 5
        assert!(circuits.iter().all(|x| x.size() < 5));

        // there should be 60 circuits of size 3
        assert_eq!(circuits.iter().filter(|x| x.size() == 3).count(), 60);

        // there should be 735 circuits of size 4
        assert_eq!(circuits.iter().filter(|x| x.size() == 4).count(), 735);
    }

    #[test]
    fn known_derived() {
        let uniform = UniformMatroid::new(5, 6);
        let res = UniformMatroid::new(1, 1);

        let derived = CombinatorialDerived::from(&uniform);

        assert!(res.is_equal(&derived))
    }

    #[test]
    fn uniform_general() {
        let matroid = UniformMatroid::new(2, 5);

        let derived = CombinatorialDerived::from(&matroid);
        let derived_uniform = CombinatorialDerived::from(&matroid);

        assert!(derived.is_equal(&derived_uniform));
    }

    #[test]
    fn uniform_2_5_from_circuits() {
        let matroid = UniformMatroid::new(2, 5);

        let derived = CombinatorialDerived::from_matroid(&matroid);
        let derived_uniform = CombinatorialDerived::from(&matroid);

        assert!(derived.is_equal(&derived_uniform));
    }

    #[test]
    fn inclusion_minimal_1() {
        let mut a: Vec<Set> = vec![0b0111.into(), 0b1111.into(), 0b1110.into()];
        a = inclusion_minimal(&a);

        let b: Vec<Set> = vec![0b0111.into(), 0b1110.into()];

        assert!(contains_same_elems!(a, b))
    }

    #[test]
    fn epsilon_1() {
        let dependents = vec![0b0111.into(), 0b1110.into()];
        let res = epsilon(&dependents, 3);

        let expected: Vec<Set> = vec![0b0111.into(), 0b1110.into(), 0b1101.into(), 0b1011.into()];

        assert!(contains_same_elems!(res, expected))
    }


    #[test]
    fn uniform_2_6() {
        // this matroid is fast, but has nullity 4, so there are dependent sets that are not in
        // A_0. However, all circuits should be in A_0, so the following calculations should yield
        // the same results
        let matroid = UniformMatroid::new(2, 6);

        let fast_calculation = CombinatorialDerived::from_fast_matroid(&matroid);
        let non_fast_calculation = CombinatorialDerived::from_non_fast_matroid(&matroid);

        assert!(fast_calculation.is_equal(&non_fast_calculation));
    }
}
