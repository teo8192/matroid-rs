use matroids::matroid::{Matroid, Vamos};

fn main() {
    let matroid = Vamos::new();

    println!("Vamos matroid:");
    for i in 1..=(matroid.n() - matroid.k()) {
        println!("{} {}", i, matroid.generalized_hamming_distance(i).unwrap());
    }

    let dual = matroid.dual();

    println!("Dual Vamos matroid:");
    for i in 1..=(dual.n() - dual.k()) {
        println!("{} {}", i, dual.generalized_hamming_distance(i).unwrap());
    }
}
