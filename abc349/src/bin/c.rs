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
        s:Chars,t:Chars
    }
    let mut mid: Vec<usize> = Vec::new();
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for i in 0..s.len() {
        if s[i] == t[0].to_lowercase().next().unwrap() {
            left.push(i);
        }
        if s[i] == t[1].to_lowercase().next().unwrap() {
            mid.push(i);
        }
        if s[i] == t[2].to_lowercase().next().unwrap() {
            right.push(i);
        }
    }

    for i in 0..left.len() {
        let m = mid.upper_bound(&left[i]);
        if m >= mid.len() {
            println!("No");
            return;
        }
        if m < mid.len() && t[2] == 'X' {
            println!("Yes");
            return;
        }
        let r = right.upper_bound(&mid[m]);
        if r < right.len() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
