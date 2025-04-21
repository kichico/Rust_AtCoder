#[allow(unused_imports)]
use itertools::*;
use memoise::memoise_map;
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
#[memoise_map(n)]
fn calc(n: i64, a: i64, x: i64, y: i64) -> f64 {
    if n == 0 {
        return 0f64;
    } else {
        let ans0 = calc(n / a, a, x, y) + x as f64;
        let ans1 = ((2..=6)
            .into_iter()
            .map(|i| calc(n / i, a, x, y))
            .sum::<f64>()
            + 6f64 * y as f64)
            / 5f64;
        return ans0.min(ans1);
    }
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:i64,a:i64,x:i64,y:i64
    }
    let ans = calc(n, a, x, y);
    println!("{}", ans);
}
fn main() {
    solve();
}
