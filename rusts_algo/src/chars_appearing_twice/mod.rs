
use std::collections::HashSet;
type S = HashSet<char>;

pub fn exec(a_orig:&[char], b_orig:&[char], c_orig:&[char]) -> S {
    assert!(a_orig.len() == 3);
    assert!(b_orig.len() == 3);
    assert!(c_orig.len() == 3);
    let a:S = HashSet::from_iter(a_orig.iter().cloned());
    let b:S = HashSet::from_iter(b_orig.iter().cloned());
    let c:S = HashSet::from_iter(c_orig.iter().cloned());
    let ab_inter:S = a.intersection(&b).cloned().collect();
    let bc_inter:S = b.intersection(&c).cloned().collect();
    let ca_inter:S = c.intersection(&a).cloned().collect();
    let abbc_union:S = ab_inter.union(&bc_inter).cloned().collect();
    let abbcca_union:S = abbc_union.union(&ca_inter).cloned().collect();
    let abc_inter:S = ab_inter.intersection(&bc_inter).cloned().collect();
    let r:S = abbcca_union.difference(&abc_inter).cloned().collect();
    println!("((A & B) | (B & C) | (C & A)) - (A & B & C) = {:?} - {:?} = {:?}", abbcca_union, abc_inter, r);
    r
}

pub fn run() {
    let a = ['A', 'B', 'C'];
    let b = ['B', 'C', 'D'];
    let c = ['C', 'D', 'E'];
    exec(&a, &b, &c);
}