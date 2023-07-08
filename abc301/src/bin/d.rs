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
        s:Chars,n:usize
    }
    let mut ans = s
        .iter()
        .enumerate()
        .map(|x| {
            if x.1 == &'1' {
                2.pow((s.len() - 1 - x.0) as u32)
            } else {
                0
            }
        })
        .sum::<i64>();
    if ans > n as i64 {
        println!("-1");
        return;
    }
    for i in (0..s.len()).rev() {
        if s[s.len() - i - 1] == '?' && ans + 2.pow(i as u32) <= n as i64 {
            ans += 2.pow(i as u32);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
