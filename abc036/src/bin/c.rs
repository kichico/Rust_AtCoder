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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut a:[usize;n]
    }
    let st = BTreeSet::from_iter(a.clone().into_iter());
    let mut dic = BTreeMap::new();
    for (i, v) in enumerate(st) {
        dic.insert(v, i);
    }
    println!("{}", a.into_iter().map(|x| dic.get(&x).unwrap()).join("\n"));
}
fn main() {
    solve();
}
