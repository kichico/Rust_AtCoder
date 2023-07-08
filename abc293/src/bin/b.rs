#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
use std::iter::FromIterator;
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
        n:usize,a:[Usize1;n]
    }
    let mut notcalled: BTreeSet<usize> = BTreeSet::from_iter(0..n);
    for i in 0..n {
        if notcalled.contains(&i) {
            notcalled.remove(&a[i]);
        }
    }
    println!("{}", notcalled.len());
    println!("{}", notcalled.iter().map(|x| x + 1).join(" "));
}
fn main() {
    solve();
}
