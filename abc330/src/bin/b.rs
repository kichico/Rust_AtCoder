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
        n:usize,l:i64,r:i64,a:[i64;n]
    }
    let mut ans: Vec<i64> = Vec::new();
    for Ai in a {
        if l - Ai > 0 {
            ans.push(l);
        } else if r - Ai < 0 {
            ans.push(r);
        } else {
            ans.push(Ai);
        }
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
