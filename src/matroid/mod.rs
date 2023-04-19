//! The module for everything related to matroids.

#[allow(clippy::module_inception)]
mod matroid;

mod bases_matroid;
mod combinatorial_derived;
mod dual;
mod elongate;
pub mod examples;
mod matrix_matroid;
mod storage;
mod uniform;
mod vamos;

pub use bases_matroid::BasesMatroid;
pub use combinatorial_derived::CombinatorialDerived;
pub use dual::Dual;
pub use elongate::Elongate;
pub use matrix_matroid::MatrixMatroid;
pub use matroid::{load_matroid, Matroid};
pub use uniform::UniformMatroid;
pub use vamos::Vamos;
