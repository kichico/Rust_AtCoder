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
        n:usize,mut alien:[usize;n]
    }
    let mut timer = vec![0usize; 1e7 as usize];
    let mut stone = 0;
    for i in 0..n {
        alien[i] += stone;
        timer[i + alien[i]] += 1;
        stone += 1;
        stone -= timer[i];
        alien[i] = alien[i].saturating_sub(n - i - 1);
    }
    println!("{}", alien.iter().join(" "));
}
fn main() {
    solve();
}
