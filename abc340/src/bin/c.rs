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
        n:usize
    }
    let mut que = BTreeMap::new();
    que.insert(n, 1);
    let mut ans = 0;
    while let Some((x, val)) = que.pop_last() {
        let lower = x / 2;
        let higher = (x + 1) / 2;
        if lower >= 2 {
            let e = que.entry(lower).or_insert(0);
            *e += val;
        }
        if higher >= 2 {
            let e = que.entry(higher).or_insert(0);
            *e += val;
        }
        ans += x * val;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
