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
        n:i64
    }
    let mut ans = 0;
    let mut kai = vec![0; n as usize];
    for ab in 1..n {
        for a in 1..ab.sqrt() + 1 {
            if ab % a != 0 {
                continue;
            }
            let b = ab / a;
            if a != b {
                kai[ab as usize] += 2;
            } else {
                kai[ab as usize] += 1;
            }
        }
    }
    for i in 1..n as usize {
        ans += kai[i] * kai[n as usize - i];
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
