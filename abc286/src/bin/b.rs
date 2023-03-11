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
        n:usize,s:Chars
    }
    let mut now = 0;
    let mut ans: Vec<char> = Vec::new();
    while now + 1 < n {
        if s[now] == 'n' && s[now + 1] == 'a' {
            ans.extend(['n', 'y', 'a'].iter());
            now += 2;
        } else {
            ans.push(s[now]);
            now += 1;
        }
    }
    if now < n {
        ans.push(s[now]);
    }
    println!("{}", ans.iter().join(""));
}
fn main() {
    solve();
}
