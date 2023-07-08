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
        n:usize,m:i128
    }
    let mut ans = m as i128;
    let MOD = 998244353;
    for _i in 1..n - 1 {
        ans *= (m - 1) as i128;
        ans = ans.rem_euclid(MOD);
    }
    //ans *= (m - 2).max(1);
    ans *= (m - 2).max(1);
    if m > 3 {
        ans *= m - 1;
    }
    println!("{}", ans.rem_euclid(MOD));
}
fn main() {
    solve();
}
