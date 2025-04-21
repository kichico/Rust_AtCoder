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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
use btreemultimap::*;
use std::ops::Bound::{Excluded, Included};
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,mut a:[usize;n]
    }
    a.sort();
    let mut ans = 0;
    for i in 0..n {
        let mut num = a.lower_bound(&(a[i] + m));

        ans = ans.max(num - i);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
