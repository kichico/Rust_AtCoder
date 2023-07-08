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
        n:usize,m:usize,a:[(i64,usize);n],b:[(i64,usize);m]
    }
    let mut ans = 0;
    let mut it_a = 0;
    let mut it_b = 0;
    while it_a < n && it_b < m {
        let dist = a[it_a].1.min(b[it_b].1);
        if a[it_a].0 == b[it_b].0 {
            ans += dist;
        }
        if a[it_a].1 > b[it_b].1 {
            it_b += 1;
        }
    }
}
fn main() {
    solve();
}
