#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use omniswap::swap;
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
        k:usize,mut s:Chars,mut t:Chars
    }
    if s.len() > t.len() {
        swap!(&mut s, &mut t);
    }
    if s.len() == t.len() {
        let mut cnt = 0;
        for (a, b) in s.iter().zip(t) {
            if *a != b {
                cnt += 1;
            }
        }
        let ans = if cnt > 1 { "No" } else { "Yes" };
        println!("{}", ans);
    } else if s.len() + 1 == t.len() {
        let mut front = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                break;
            }
            front += 1;
        }
        s.reverse();
        t.reverse();
        let mut back = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                break;
            }
            back += 1;
        }
        let ans = if front + back >= s.len() { "Yes" } else { "No" };
        println!("{}", ans);
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
