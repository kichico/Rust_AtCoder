#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
        n:usize,mut a:[i64;n]
    }
    a.sort();
    let sum = a.iter().cumsum().collect::<Vec<i64>>();
    let nijyo = a.iter().map(|x| x * x).cumsum().collect::<Vec<i64>>();
    let mut ans = 0;
    for i in 1..n {
        ans += a[i] * a[i] * i as i64 + nijyo[i - 1] - 2 * a[i] * sum[i - 1];
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
