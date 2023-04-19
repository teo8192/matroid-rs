use matroids::matroid::{load_matroid, Matroid};
use std::path::Path;

fn main() {
    let vamos_derived = load_matroid(Path::new("calculated_matroids/vamos_derived")).unwrap();
    println!("Matroid loaded...");
    println!("Rank: {}", vamos_derived.k());
    println!("N: {}", vamos_derived.n());
    println!("Vamos derived matroid has {} circuits", vamos_derived.par_circuits().len());
}
