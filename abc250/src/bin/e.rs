use ac_library::{segtree::*, MapMonoid, Monoid};
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
struct Xor;
impl Monoid for Xor {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[usize;n],b:[usize;n],q:usize,kukan:[(Usize1,Usize1);q]
    }
    let mut aseg = Segtree::<Xor>::new(n);
    let mut bseg = Segtree::<Xor>::new(n);
    for i in 0..n {
        aseg.set(i, a[i]);
        bseg.set(i, b[i]);
    }
    for (x, y) in kukan {
        let a_xor = aseg.prod(0..=x);
        let b_xor = bseg.prod(0..=y);
        println!("a_xor:{} b_xor:{}", a_xor, b_xor);
        if a_xor == b_xor {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
fn main() {
    solve();
}
