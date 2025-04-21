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
fn solve() {
    input! {
        n:usize,points:[(i64,i64);n]
    }
    let mut dist = 0f64;
    let mut x = 0;
    let mut y = 0;
    for (p, q) in points {
        dist += ((x - p).pow(2) as f64 + (y - q).pow(2) as f64).sqrt();
        x = p;
        y = q;
    }
    dist += (x.pow(2) as f64 + y.pow(2) as f64).sqrt();
    println!("{}", dist);
}
fn main() {
    solve();
}
