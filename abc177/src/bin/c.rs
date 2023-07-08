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
        n:usize,a:[i64;n]
    }
    let MOD = 1e9 as i64 + 7;
    let mut sum: i64 = a.iter().sum();
    sum = sum.rem_euclid(MOD);
    let mut ans = 0;
    for i in 0..n {
        sum -= a[i];
        ans += sum * a[i];
        ans = ans.rem_euclid(MOD);
    }
    println!("{}", ans.rem_euclid(MOD));
}
fn main() {
    solve();
}
