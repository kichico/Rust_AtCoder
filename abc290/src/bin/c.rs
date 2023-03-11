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
        n:usize,k:usize,mut a:[i64;n]
    }
    a.sort();
    let a = a.into_iter().dedup().collect::<Vec<i64>>();
    let mut ans = 0;
    let len = std::cmp::min(a.len(), k);
    let a = a.iter().take(len);
    for i in a {
        if ans != *i {
            println!("{}", ans);
            return;
        } else {
            ans += 1;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
