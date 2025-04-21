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
        n:usize,mut a:[(i64,i64);n]
    }
    a.sort_by(|x, y| x.0.cmp(&y.0));
    let mut ans = if n % 2 == 0 {
        -(a[n / 2].0 + a[n / 2 - 1].0)
    } else {
        -a[n / 2].0
    };
    a.sort_by(|x, y| x.1.cmp(&y.1));
    ans += if n % 2 == 0 {
        a[n / 2].1 + a[n / 2 - 1].1 + 1
    } else {
        a[n / 2].1 + 1
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
