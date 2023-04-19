use log::info;
use matroids::matroid::{Matroid, Vamos};
use simple_logger::SimpleLogger;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Starting vamos_derived_calculation");

    let vamos = Vamos::new();

    let matroid = vamos.combinatorial_derived();
    matroid.save(Path::new("vamos_derived")).unwrap();
    println!("Got derived vamos matroid of rank: {}", matroid.k());
    println!("It has {} circuits...", matroid.par_circuits().len());

    for i in 1..=(matroid.n() - matroid.k()) {
        println!("d_{}: {:?}", i, matroid.generalized_hamming_distance(i));
    }
}
