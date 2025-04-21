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
        n:usize,a:[usize;n]
    }
    if n <= 2 {
        println!("1");
        return;
    }
    let mut left = vec![0; n];
    let mut right = left.clone();
    left[0] = 1;
    for i in 1..n {
        left[i] = a[i].min(left[i - 1] + 1);
    }
    right[n - 1] = 1;
    for i in (0..n - 1).rev() {
        right[i] = a[i].min(right[i + 1] + 1)
    }
    let ans = left
        .iter()
        .zip(right)
        .map(|(&x, y)| x.min(y))
        .max()
        .unwrap();
    println!("{}", ans);
}
fn main() {
    solve();
}
