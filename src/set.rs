use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

/// A set of elements
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Hash)]
pub struct Set {
    content: usize,
}

impl Set {
    /// Create an empty set
    pub fn empty() -> Self {
        Self { content: 0 }
    }

    /// Create a set with all elements of size n, (all the n rightmost elements)
    /// If we want a set of 5 elements:
    /// ```
    /// use matroids::set::Set;
    /// let set = Set::of_size(5);
    /// assert_eq!(set.size(), 5);
    /// ```
    pub fn of_size(n: usize) -> Self {
        Set {
            content: (1 << n) - 1,
        }
    }

    #[inline]
    /// the "index" of the leftmost element in the set
    ///
    /// as an example:
    /// ```
    /// use matroids::set::Set;
    /// let set = Set::from(0b1001);
    /// assert_eq!(set.leftmost_element(), 3);
    /// ```
    pub fn leftmost_element(&self) -> usize {
        (self.content as f32).log2() as usize
    }

    #[inline]
    /// the size/cardinality of the set
    pub fn size(&self) -> usize {
        self.content.count_ones() as usize
    }

    #[inline]
    /// calculate self ∪ other
    pub fn union(&self, other: &Self) -> Self {
        Set {
            content: self.content | other.content,
        }
    }

    #[inline]
    /// calculate self ∩ other
    pub fn intersect(&self, other: &Self) -> Self {
        Set {
            content: self.content & other.content,
        }
    }

    /// Calculate self - other
    #[inline]
    pub fn difference(&self, other: &Self) -> Self {
        Set {
            content: self.content & !other.content,
        }
    }

    #[inline]
    /// Calculate self ⊕ other = (self ∪ other) - (self ∩ other)
    ///
    /// A demonstration of the fact:
    /// ```
    /// use matroids::set::Set;
    /// let set1 = Set::from(0b1001);
    /// let set2 = Set::from(0b0111);
    /// assert_eq!(set1.symmetric_difference(&set2), set1.union(&set2).difference(&set1.intersect(&set2)));
    /// ```
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Set {
            content: self.content ^ other.content,
        }
    }

    #[inline]
    /// removes the specified element from the set
    /// element has to be the index in the set
    pub fn remove_element(&self, element: usize) -> Self {
        Set {
            content: self.content & !(1 << element),
        }
    }

    #[inline]
    /// adds the specified element to the set
    /// element has to be the index in the set
    pub fn add_element(&self, element: usize) -> Self {
        Set {
            content: self.content | (1 << element),
        }
    }

    #[inline]
    /// returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        self.content == 0
    }

    #[inline]
    /// returns true if the set containes the element
    /// element has to be the index in the set
    pub fn contains_element(&self, element: usize) -> bool {
        self.content & (1 << element) != 0
    }

    /// If self is a subset of set, then extend self to be of the format of set
    /// assumes that self.size() <= set.size()
    pub fn extend(&self, set: &Self) -> Self {
        debug_assert!(self.size() <= set.size());

        let s = self.leftmost_element();
        let k = set.leftmost_element();
        let mut content = 0;
        let mut i = 0;
        let mut j = 0;
        while i <= s && j <= k {
            // if the j'th bit of set is set
            if (set.content >> j) & 1 == 1 {
                // then add the i'th bit of self at the j'th position
                content |= ((self.content >> i) & 1) << j;
                i += 1;
            }
            j += 1;
        }

        Self { content }
    }

    /// Take the union of the sets that are chosen by self
    pub fn union_of_sets(&self, sets: &[Set]) -> Self {
        (0..=self.leftmost_element())
            .filter(|i| self.contains_element(*i))
            .fold(Set::empty(), |acc, i| acc.union(&sets[i]))
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.content)
    }
}

impl PartialEq<&Set> for Set {
    fn eq(&self, other: &&Set) -> bool {
        self.content == other.content
    }
}

impl PartialEq<Set> for &Set {
    fn eq(&self, other: &Set) -> bool {
        self.content == other.content
    }
}

// {{{ From implementations

impl From<usize> for Set {
    fn from(content: usize) -> Self {
        Set { content }
    }
}

impl From<&usize> for Set {
    fn from(content: &usize) -> Self {
        Set { content: *content }
    }
}

impl From<Set> for usize {
    fn from(s: Set) -> Self {
        s.content
    }
}

impl From<&Set> for usize {
    fn from(s: &Set) -> Self {
        s.content
    }
}

impl From<Vec<usize>> for Set {
    fn from(content: Vec<usize>) -> Self {
        Set {
            content: content.into_iter().fold(0, |acc, x| acc | (1 << x)),
        }
    }
}

impl From<&[usize]> for Set {
    fn from(content: &[usize]) -> Self {
        Set {
            content: content.iter().fold(0, |acc, x| acc | (1 << x)),
        }
    }
}

impl<const N: usize> From<[usize; N]> for Set {
    fn from(content: [usize; N]) -> Self {
        Set {
            content: content.iter().fold(0, |acc, x| acc | (1 << x)),
        }
    }
}

impl<const N: usize> From<&[usize; N]> for Set {
    fn from(content: &[usize; N]) -> Self {
        Set {
            content: content.iter().fold(0, |acc, x| acc | (1 << x)),
        }
    }
}

impl From<&Set> for Vec<usize> {
    fn from(set: &Set) -> Self {
        let mut content = set.content;
        let mut result = Vec::new();
        let mut i = 0;
        while content > 0 {
            if content & 1 == 1 {
                result.push(i);
            }
            content >>= 1;
            i += 1;
        }
        result
    }
}

impl From<Set> for Vec<usize> {
    fn from(set: Set) -> Self {
        (&set).into()
    }
}

// }}}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.content == other.content {
            Some(Ordering::Equal)
        } else if self.intersect(other) == self {
            Some(Ordering::Less)
        } else if self.intersect(other) == other {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[allow(unused)]
enum LimitPolicy {
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
}

/// Iterate over sets
pub struct SetIterator {
    current: usize,
    n: usize,
    size_limit: Option<usize>,
    size_limit_policy: Option<LimitPolicy>,
}

impl SetIterator {
    /// Creates a new iterator over all subsets of a set of size `n`.
    /// After this is created, you can specify a size limit for the subsets iterated over.
    /// Then a limit policy can be specified, which specifies how the size limit is interpreted.
    /// Example of iterating though all subsets of size 3 of a set of size 5:
    /// ```
    /// use matroids::set::SetIterator;
    /// let mut iter = SetIterator::new(5).size_limit(3).equal();
    /// assert_eq!(iter.next(), Some(0b111.into()));
    /// assert_eq!(iter.next(), Some(0b1011.into()));
    /// assert_eq!(iter.next(), Some(0b1101.into()));
    /// ```
    pub fn new(n: usize) -> Self {
        if n > usize::BITS as usize {
            panic!(
                "tried to create a set iterator on {} elements, but the maximal supported are {}",
                n,
                usize::BITS
            );
        }
        SetIterator {
            current: 0,
            n,
            size_limit: None,
            size_limit_policy: None,
        }
    }

    /// Set the size of the subsets iterated over to be at most `size_limit`.
    #[allow(unused)]
    pub fn size_limit(mut self, size_limit: usize) -> Self {
        self.size_limit = Some(size_limit);
        self.size_limit_policy = Some(LimitPolicy::Equal);
        self
    }

    /// Set the size of the subsets iterated over to be equal to `size_limit`.
    #[allow(unused)]
    pub fn equal(mut self) -> Self {
        self.size_limit_policy = Some(LimitPolicy::Equal);
        self
    }

    /// iterate over subsets of size strictly smaller than `size_limit`.
    #[allow(unused)]
    pub fn smaller(mut self) -> Self {
        self.size_limit_policy = Some(LimitPolicy::Less);
        self
    }

    /// iterate over subsets of size smaller or equal to `size_limit`.
    #[allow(unused)]
    pub fn smaller_equal(mut self) -> Self {
        self.size_limit_policy = Some(LimitPolicy::LessEqual);
        self
    }

    /// iterate over subsets of size strictly greater than `size_limit`.
    #[allow(unused)]
    pub fn greater(mut self) -> Self {
        self.size_limit_policy = Some(LimitPolicy::Greater);
        self
    }

    /// iterate over subsets of size greater or equal to `size_limit`.
    #[allow(unused)]
    pub fn greater_equal(mut self) -> Self {
        self.size_limit_policy = Some(LimitPolicy::GreaterEqual);
        self
    }

    fn satisfy_limit(&self, item: usize) -> bool {
        let size = item.count_ones() as usize;
        match self.size_limit_policy {
            Some(LimitPolicy::Less) => size < self.size_limit.unwrap(),
            Some(LimitPolicy::LessEqual) => size <= self.size_limit.unwrap(),
            Some(LimitPolicy::Equal) => size == self.size_limit.unwrap(),
            Some(LimitPolicy::GreaterEqual) => size >= self.size_limit.unwrap(),
            Some(LimitPolicy::Greater) => size > self.size_limit.unwrap(),
            None => true,
        }
    }

    fn set_next(&mut self) -> Option<Set> {
        match self.size_limit_policy {
            Some(LimitPolicy::Equal) => {
                self.size_limit.and_then(|limit| {
                    if self.current == 0 && limit > 0 {
                        self.current = (1 << limit) - 1;
                        Some(Set {
                            content: self.current,
                        })
                    } else if self.current >= 1 << self.n {
                        None
                    } else if limit == 0 {
                        self.current = 1 << self.n;
                        Some(Set { content: 0 })
                    } else {
                        // need to find next
                        // the idea here is to find the first place where I may move an lement to
                        // the left, and then reset all elements to the right of it
                        let mut i = 0;
                        // want to find the pattern *..**011..1100..00, and move the leftmost 1
                        // once to the left and reset all elements to the right of it
                        while (self.current >> i) & 3 != 1 {
                            i += 1;
                        }
                        // move the 1 to the left
                        self.current ^= 3 << i;
                        // find stuff to the right (to be able to count them)
                        let stuff_to_right = self.current & ((1 << i) - 1);
                        // remove stuff to the right
                        self.current &= !((1 << i) - 1);
                        // add stuff to the right
                        self.current |= (1 << stuff_to_right.count_ones()) - 1;

                        if self.current >= 1 << self.n {
                            None
                        } else {
                            Some(Set {
                                content: self.current,
                            })
                        }
                    }
                })
            }
            _ => {
                while !self.satisfy_limit(self.current) {
                    self.current += 1;
                    if self.current >= 1 << self.n {
                        return None;
                    }
                }
                let result = Set {
                    content: self.current,
                };
                self.current += 1;
                Some(result)
            }
        }
    }
}

impl Iterator for SetIterator {
    type Item = Set;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 1 << self.n {
            return None;
        }
        self.set_next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        let a = Set::from(0b101);
        let b = Set::from(0b101);

        assert_eq!(a, b);
    }

    #[test]
    fn ordering() {
        let a = Set::from(0b11101);
        let b = Set::from(0b00101);
        let c = Set::from(0b10011);

        assert!(b < a);
        assert!(b <= a);
        assert!(!(a > c));
        assert!(!(a < c));
        assert!(!(b < c));
        assert!(!(b > c));
    }

    #[test]
    fn intersect() {
        let a = Set::from(0b101);
        let b = Set::from(0b110);
        let c = Set::from(0b100);

        assert_eq!(a.intersect(&b), c);
    }

    #[test]
    fn union() {
        let a = Set::from(0b101);
        let b = Set::from(0b110);
        let c = Set::from(0b111);

        assert_eq!(a.union(&b), c);
    }

    #[test]
    fn leftmost() {
        let a = Set::from(0b101);
        let b = Set::from(0b001);
        let c = Set::from(0b1000);

        assert_eq!(a.leftmost_element(), 2);
        assert_eq!(b.leftmost_element(), 0);
        assert_eq!(c.leftmost_element(), 3);
    }

    #[test]
    fn extend() {
        let a = Set::from(0b11101);
        let b = Set::from(0b00101);
        let c = Set::from(0b01001);

        assert_eq!(b.extend(&a), c);
    }

    #[test]
    fn extend_single_elem() {
        let a = Set::from(0b11101);
        let b = Set::from(0b00100);
        let c = Set::from(0b01000);

        assert_eq!(b.extend(&a), c);
    }

    #[test]
    fn extend_single_elem_base() {
        let a = Set::from(0b10000);
        let b = Set::from(0b00001);
        let c = Set::from(0b10000);

        assert_eq!(b.extend(&a), c);
    }

    #[test]
    fn iterator_all() {
        let mut iter = SetIterator::new(3);
        assert_eq!(iter.next(), Some(Set::from(0b000)));
        assert_eq!(iter.next(), Some(Set::from(0b001)));
        assert_eq!(iter.next(), Some(Set::from(0b010)));
        assert_eq!(iter.next(), Some(Set::from(0b011)));
        assert_eq!(iter.next(), Some(Set::from(0b100)));
        assert_eq!(iter.next(), Some(Set::from(0b101)));
        assert_eq!(iter.next(), Some(Set::from(0b110)));
        assert_eq!(iter.next(), Some(Set::from(0b111)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_equal() {
        let mut iter = SetIterator::new(6).size_limit(3).equal();
        assert_eq!(iter.next(), Some(Set::from(0b000111)));
        assert_eq!(iter.next(), Some(Set::from(0b001011)));
        assert_eq!(iter.next(), Some(Set::from(0b001101)));
        assert_eq!(iter.next(), Some(Set::from(0b001110)));
        assert_eq!(iter.next(), Some(Set::from(0b010011)));
        assert_eq!(iter.next(), Some(Set::from(0b010101)));
        assert_eq!(iter.next(), Some(Set::from(0b010110)));
        assert_eq!(iter.next(), Some(Set::from(0b011001)));
        assert_eq!(iter.next(), Some(Set::from(0b011010)));
        assert_eq!(iter.next(), Some(Set::from(0b011100)));
        assert_eq!(iter.next(), Some(Set::from(0b100011)));
        assert_eq!(iter.next(), Some(Set::from(0b100101)));
        assert_eq!(iter.next(), Some(Set::from(0b100110)));
        assert_eq!(iter.next(), Some(Set::from(0b101001)));
        assert_eq!(iter.next(), Some(Set::from(0b101010)));
        assert_eq!(iter.next(), Some(Set::from(0b101100)));
        assert_eq!(iter.next(), Some(Set::from(0b110001)));
        assert_eq!(iter.next(), Some(Set::from(0b110010)));
        assert_eq!(iter.next(), Some(Set::from(0b110100)));
        assert_eq!(iter.next(), Some(Set::from(0b111000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn size() {
        let count = SetIterator::new(41).size_limit(4).equal().count();

        // this should be equal to 41 choose 4
        assert_eq!(count, 101270);
    }
}
