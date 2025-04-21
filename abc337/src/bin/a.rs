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
        n:usize,a:[(usize,usize);n]
    }
    let taka = a.iter().map(|x| x.0).sum::<usize>();
    let aoki = a.iter().map(|x| x.1).sum::<usize>();
    let ans = if taka > aoki {
        "Takahashi"
    } else if taka < aoki {
        "Aoki"
    } else {
        "Draw"
    };
    println!("{}", ans);
}
fn main() {
    solve();
}
