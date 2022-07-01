

pub mod appear_twice {

    use std::collections::BTreeSet;

    pub fn exec() -> BTreeSet<char> {
        let a = BTreeSet::from_iter(['a', 'b', 'c']);
        let b = BTreeSet::from_iter(['b', 'c', 'd']);
        let c = BTreeSet::from_iter(['c', 'd', 'e']);
        let ab:BTreeSet<char> = a.intersection(&b).cloned().collect();
        let bc:BTreeSet<char> = b.intersection(&c).cloned().collect();
        let ca:BTreeSet<char> = c.intersection(&a).cloned().collect();
        let abbc:BTreeSet<char> = ab.union(&bc).cloned().collect();
        let abbcca:BTreeSet<char> = abbc.union(&ca).cloned().collect();
        let ab_int:BTreeSet<char> = a.intersection(&b).cloned().collect();
        let abc_int:BTreeSet<char> = ab_int.intersection(&c).cloned().collect();
        let final_set:BTreeSet<char> = abbcca.difference(&abc_int).cloned().collect();
        println!("A - {:?}, b - {:?}, c - {:?}", a, b, c);
        println!("A&B - {:?}", ab);
        println!("B&C - {:?}", bc);
        println!("C&A - {:?}", ca);
        println!("(A&B) | (B&C) | (C&A) - {:?}", abbcca); 
        println!("(A&B&C) - {:?}", abc_int);
        println!("(A&B | B&C | C&A) - (A&B&C) - {:?}", final_set);

        final_set
    }

    pub fn run() {
        exec();
    }
}