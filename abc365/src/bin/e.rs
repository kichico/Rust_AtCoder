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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn to_Binary(mut x: usize, keta: usize) -> VecDeque<char> {
    let mut v: VecDeque<char> = VecDeque::new();
    if x == 0 {
        v.push_front('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push_front(c);
        x /= 2;
    }
    while v.len() < keta {
        v.push_front('0');
    }
    return v;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[usize;n]
    }
    let binaries = a.iter().map(|x| to_Binary(*x, 30)).collect::<Vec<_>>();
}
fn main() {
    solve();
}
