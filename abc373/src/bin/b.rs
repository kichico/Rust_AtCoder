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
        s:Chars
    }
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let mut mp = HashMap::new();
    for i in 0..26usize {
        mp.insert(s[i], i);
    }
    let mut dist = 0;
    let mut now = *mp.get(&t[0]).unwrap();
    for i in 1..26 {
        dist += mp.get(&t[i]).unwrap().abs_diff(now);
        now = *mp.get(&t[i]).unwrap();
    }
    println!("{}", dist);
}
fn main() {
    solve();
}
