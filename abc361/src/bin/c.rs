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
        n:usize,k:usize,mut a:[usize;n]
    }
    a.sort();
    let mut a: VecDeque<usize> = a.into_iter().collect::<VecDeque<usize>>();
    for i in 0..k {
        let front = a[1] - a[0];
        let back = a[a.len() - 1] - a[a.len() - 2];
        if front > back {
            a.pop_front();
        } else {
            a.pop_back();
        }
    }
    println!("{}", a.iter().last().unwrap() - a[0]);
}
fn main() {
    solve();
}
