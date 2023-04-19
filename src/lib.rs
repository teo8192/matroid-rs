//! This is a library for working with matroids.
//!
//! There is an optional feature, `progress`, which shows progress bars when calculating the
//! combinatorial derived of non-fast matroids. Warning: This slows the code significantly.
//!
//! # Examples
//!
//! Calculation of the betti numbers of a uniform matroid on 5 elements of rank 3:
//! ```
//! use matroids::matroid::{UniformMatroid, Matroid};
//!
//! let matroid = UniformMatroid::new(3, 5);
//! let betti = matroid.betti();
//! // will print the free resolution with latex formatting
//! println!("{}", betti);
//! ```
//!
//! Calculation of the combinatorial derived matroid
//! ```
//! use matroids::matroid::{UniformMatroid, Matroid};
//!
//! let matroid = UniformMatroid::new(3, 5);
//! let derived = matroid.combinatorial_derived();
//!
//! assert_eq!(derived.circuits().len(), 10);
//! ```
//!
//! The Vamos matroid
//! ```
//! use matroids::matroid::{Vamos, Matroid};
//!
//! let matroid = Vamos::new();
//!
//! assert_eq!(matroid.k(), 4);
//! assert_eq!(matroid.n(), 8);
//! ```
//!
//! Duals:
//! ```
//! use matroids::matroid::{MatrixMatroid, Matroid};
//! use matroids::matrix::{DynMatrix, Matrix};
//! use tinyfield::prime_field::PrimeField;
//! use tinyfield::GF2;
//!
//! let zer = GF2::zero;
//! let one = GF2::one;
//!
//! // the generator matrix for the hamming code Ham(7, 4)
//! let matrix = DynMatrix::from_rows(&[
//!     &[one, zer, zer, zer, zer, one, one],
//!     &[zer, one, zer, zer, one, zer, one],
//!     &[zer, zer, one, zer, one, one, zer],
//!     &[zer, zer, zer, one, one, one, one],
//! ]).unwrap();
//!
//! let matroid = MatrixMatroid::from(matrix);
//! let dual = matroid.dual();
//! let derived = matroid.combinatorial_derived();
//!
//! // it happens that the combinatorial derived of Ham(7,4) is the same as the dual
//! assert!(derived.is_equal(&dual));
//! ```

extern crate postcard;
extern crate rayon;
extern crate serde;
extern crate tinyfield;

pub mod matrix;
pub mod matroid;
pub mod betti_nums;
pub mod set;

mod utils;
mod field;
