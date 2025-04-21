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
        n:usize,m:usize,a:[Usize1;m]
    }
    let mut vote: BTreeSet<usize> = BTreeSet::new();
    let mut maxi = 0;
    let mut gained = vec![0; n];
    for i in 0..m {
        gained[a[i]] += 1;
        if maxi < gained[a[i]] {
            maxi = gained[a[i]];
            vote.clear();
            vote.insert(a[i]);
        } else if maxi == gained[a[i]] {
            vote.insert(a[i]);
        }
        println!("{}", vote.first().unwrap() + 1);
    }
}
fn main() {
    solve();
}
