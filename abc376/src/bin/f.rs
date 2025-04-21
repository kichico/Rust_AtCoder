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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:i64,q:usize,query:[(char,i64);q]
    }
    let mut ans = 0;
    let left = 1;
    let right = 2;
    let mut poses = vec![left, right];
    for (hand, pos) in query {
        // increasing
        let idx = if hand == 'R' { 1 } else { 0 };
        let mut now = poses[idx];
        let mut inc = 0;
        while now != pos {
            now += 1;
            inc = if inc == i64::MAX { i64::MAX } else { inc + 1 };
            if now == poses[1 - idx] {
                inc = i64::MAX;
            }
            if now > n {
                now = 1;
            }
            if now == poses[1 - idx] {
                inc = i64::MAX;
            }
        }
        let mut now = poses[idx];
        let mut dec = 0;
        while now != pos {
            now -= 1;
            dec = if dec == i64::MAX { i64::MAX } else { dec + 1 };
            if now == poses[1 - idx] {
                dec = i64::MAX;
            }
            if now < 1 {
                now = n;
            }
            if now == poses[1 - idx] {
                dec = i64::MAX;
            }
        }
        ans += dec.min(inc);
        poses[idx] = pos;
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
