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
        n:usize,d:usize,x:[[i64;d];n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if i == j {
                continue;
            }
            let mut val = 0;
            for (y, z) in zip(&x[i], &x[j]) {
                val += (y - z).pow(2);
            }
            if val == val.sqrt() * val.sqrt() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
