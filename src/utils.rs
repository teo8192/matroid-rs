#[allow(unused)]
macro_rules! contains_same_elems {
    ($a:expr, $b:expr) => {
        $a.len() == $b.len()
            && $a.iter().all(|aelem| $b.contains(aelem))
            && $b.iter().all(|belem| $a.contains(belem))
    };
}

#[allow(unused)]
pub(crate) use contains_same_elems;
