use matroids::matroid::{Matroid, UniformMatroid};

fn main() {
    for n in 1..=7 {
        for k in 1..n {
            let matroid = UniformMatroid::new(k, n).combinatorial_derived();
            // the maximal elongation is n - k
            for elongation in 0..=(matroid.n() - matroid.k()) {
                let elongated_matroid = matroid.elongate(elongation);

                println!("U_{}{}^({}): {}", k, n, elongation, elongated_matroid.betti());
            }
        }
    }
}
