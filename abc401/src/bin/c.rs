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
        n:usize,k:usize
    }
    let mut a = vec![0; n + 1];
    let mut current: i128 = 0;
    for i in 0..k.min(n + 1) {
        a[i] = 1;
        current += 1;
    }
    for i in k..=n {
        a[i] = current;
        current += a[i] - a[i - k];
        current = current.rem_euclid(1e12 as i128);
    }
    println!("{}", a[n] % 1e9 as i128);
}
fn main() {
    solve();
}
