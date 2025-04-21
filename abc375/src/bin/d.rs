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
        s:Chars
    }

    let n = s.len();
    let mut cnt: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    for i in 0..n {
        cnt.entry(s[i]).or_insert(Vec::new()).push(i);
    }
    let alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let mut ans = 0;
    for i in 1..n - 1 {
        let c = s[i];
        for j in 0..26 {
            let tgt = alphabets[j];
            if let Some(v) = cnt.get(&tgt) {
                let right = v.upper_bound(&i);
                let len = v.len();
                let right_num = len - right;
                let left_num = if c == tgt { right - 1 } else { right };
                ans += right_num * left_num;
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
