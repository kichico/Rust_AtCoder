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
        n:usize,mut f:[Chars;n]
    }
    let mut v1 = f[0].clone();
    v1.insert(0, f[1][0]);
    v1.pop();
    let mut v2 = f[n - 1].clone();
    v2.push(f[n - 2][n - 1]);
    v2.remove(0);
    for i in (0..n - 1).rev() {
        f[i + 1][n - 1] = f[i][n - 1].clone();
    }
    for i in 0..n - 1 {
        f[i][0] = f[i + 1][0].clone();
    }
    f[0] = v1;
    f[n - 1] = v2;
    for i in 0..n {
        println!("{}", f[i].iter().join(""));
    }
}
fn main() {
    solve();
}
