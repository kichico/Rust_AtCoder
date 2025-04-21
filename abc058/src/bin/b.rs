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
        s:Chars,t:Chars
    }
    let mut ans = zip(&s, &t)
        .map(|x| x.0.to_string() + &x.1.to_string())
        .collect::<Vec<String>>();
    if s.len() > t.len() {
        ans.push(s.iter().next_back().unwrap().to_string());
    }
    println!("{}", ans.iter().join(""));
}
fn main() {
    solve();
}
