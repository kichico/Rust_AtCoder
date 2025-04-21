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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize
    }
    let mut phones: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..n {
        input! {k:usize,a:[usize;k]}
        phones[i] = HashSet::from_iter(a.into_iter());
    }
    input! {p:usize,q:usize,waves:[usize;p]}
    let b = HashSet::from_iter(waves.into_iter());
    let ans = phones
        .iter()
        .filter(|x| x.intersection(&b).into_iter().count() >= q)
        .count();
    println!("{}", ans);
}
fn main() {
    solve();
}
