use matroids::matroid::{MatrixMatroid, examples, Matroid};
use matroids::matrix::{DynMatrix, Matrix};
use matroids::set::SetIterator;
use tinyfield::prime_field::PrimeField;
use tinyfield::GF2;

fn main() {
    let matroid = examples::non_fast_matroid();

    let mut matrix = DynMatrix::new(matroid.n(), matroid.circuits().len());

    for (col, c) in matroid.circuits().iter().enumerate() {
        for i in 0..matroid.n() {
            matrix[(i, col)] = if c.contains_element(i) {
                GF2::one
            } else {
                GF2::zero
            };
        }
    }

    println!("{:?}", matrix);

    let owderived = MatrixMatroid::from(matrix);

    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    for (e, circuit) in matroid.circuits().iter().enumerate() {
        let c: Vec<_> = circuit.into();
        print!("{}: ", alphabet[e]);
        for i in c {
            print!("{}", i + 1);
        }
        println!();
    }

    let combinatorial_derived = matroid.combinatorial_derived();

    for circuit in combinatorial_derived.circuits() {
        let c: Vec<_> = circuit.into();
        for i in c {
            print!("{}", alphabet[i]);
        }
        print!(", ");
    }
    println!();

    fn info<M: Matroid>(m: &M) {
        println!("Got matroid or rank: {} on {} elements", m.k(), m.n());
        println!("Got {} bases", m.bases().len());
        println!("Got {} circuits", m.circuits().len());
    }

    println!("Oxley-Wang:");
    info(&owderived);

    println!("Combinatorial derived:");
    info(&combinatorial_derived);

    let mut independent_in_ow_but_not_fjk = 0;
    let mut independent_in_fjk_but_not_ow = 0;

    for set in SetIterator::new(combinatorial_derived.n()) {
        match (
            owderived.is_independent(&set),
            combinatorial_derived.is_independent(&set),
        ) {
            (true, false) => {
                let v: Vec<_> = set.into();
                for i in v {
                    print!("{}, ", i + 1);
                }
                println!(" is independent in Oxley-Wang but not in combinatorial derived",);
                independent_in_ow_but_not_fjk += 1;
            }
            (false, true) => {
                let v: Vec<_> = set.into();
                for i in v {
                    print!("{}, ", i + 1);
                }
                println!(" is independent in combinatorial derived but not in Oxley-Wang",);
                independent_in_fjk_but_not_ow += 1;
            }
            _ => {}
        }
    }

    println!(
        "Independent in Oxley-Wang but not in combinatorial derived: {}",
        independent_in_ow_but_not_fjk
    );
    println!(
        "Independent in combinatorial derived but not in Oxley-Wang: {}",
        independent_in_fjk_but_not_ow
    );
}
