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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    let n = 26;
    input! {
        d:usize,decrease_tendency:[i64;n],s:[[i64;n];d],t:[Usize1;d]
    }
    let mut manzokudo = 0;
    let mut last_done = vec![0i64; n];
    let mut ans = Vec::new();
    for day in 0..d {
        manzokudo += s[day][t[day]];
        last_done[t[day]] = day as i64 + 1;
        for i in 0..n {
            manzokudo -= decrease_tendency[i] * (day as i64 + 1 - last_done[i]);
        }
        ans.push(manzokudo);
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
