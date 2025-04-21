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
        n:usize,p:[Usize1;n],zekken:[Usize1;n]
    }
    let mut ans = vec![0; n];
    let mut zekken2p = BTreeMap::new();
    for i in 0..n {
        *zekken2p.entry(zekken[i]).or_insert(0) = i;
    }
    for i in 0..n {
        ans[i] = zekken[p[*zekken2p.get(&i).unwrap()]] + 1;
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
