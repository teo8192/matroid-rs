use matroids::matroid::{examples, Matroid};

fn main() {
    let matroid = examples::non_fast_matroid();
    let derived = matroid.combinatorial_derived();

    let circuits = matroid.circuits();

    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for circuit in derived.circuits() {
        let nullity = matroid.nullity(&circuit.union_of_sets(&circuits));
        if circuit.size() <= nullity {
            let c: Vec<usize> = circuit.into();
            for i in c {
                print!("{}", alphabet[i]);
            }
            println!(" is a circuit, but not in A_0! (it has nullity {})", nullity);
        }
    }
}
