use crate::set::{Set, SetIterator};

use super::Matroid;

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

fn is_independent(subset: &Set) -> bool {
    use std::cmp::Ordering::*;
    // every set of cardinality smaller than 4 is independent
    // There are only 5 circuits of cardinality 4
    // The larger sets are dependent
    match subset.size().cmp(&4) {
        Less => true,
        Greater => false,
        Equal => !matches!(
            subset.into(),
            0b00111001 | 0b11001001 | 0b00001111 | 0b11000110 | 0b00110110
        ),
    }
}

/// The Vamos matroid
/// see <https://en.wikipedia.org/wiki/Vamos_matroid>
pub struct Vamos {
    bases: Vec<Set>,
}

impl Default for Vamos {
    fn default() -> Self {
        Self::new()
    }
}

impl Vamos {
    pub fn new() -> Self {
        Self {
            bases: SetIterator::new(8)
                .size_limit(4)
                .equal()
                .filter(is_independent)
                .collect(),
        }
    }
}

impl Matroid for Vamos {
    fn rank(&self, subset: &Set) -> usize {
        if subset.size() < 4 {
            return subset.size();
        }
        let mut rank = 0;
        for base in self.bases.iter() {
            rank = max!(rank, base.intersect(subset).size());
            if rank == 4 {
                break;
            }
        }

        rank
    }

    fn is_independent(&self, subset: &Set) -> bool {
        is_independent(subset)
    }

    fn k(&self) -> usize {
        4
    }

    fn n(&self) -> usize {
        8
    }

    fn is_uniform(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_number_of_bases() {
        assert_eq!(Vamos::new().bases().len(), 65);
    }
}
