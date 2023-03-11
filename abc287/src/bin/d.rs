#[allow(unused_imports)]
use itertools::Itertools;
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
        mut s:Chars,mut t:Chars
    }
    let mut prev = vec![false; s.len() + 1];
    let mut suff = vec![false; s.len() + 1];
    prev[0] = true;
    for i in 0..t.len() {
        if s[i] == '?' || t[i] == '?' || s[i] == t[i] {
            prev[i + 1] = true;
        } else {
            break;
        }
    }
    s.reverse();
    t.reverse();
    suff[0] = true;
    for i in 0..t.len() {
        if s[i] == '?' || t[i] == '?' || s[i] == t[i] {
            suff[i + 1] = true;
        } else {
            break;
        }
    }
    for x in 0..=t.len() {
        if prev[x] && suff[t.len() - x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
fn main() {
    solve();
}
