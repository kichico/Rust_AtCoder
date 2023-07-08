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
        n:usize,task:[(i64,i64);n]
    }
    let mut ans = 1e12 as i64;
    for i in 0..n {
        for j in 0..n {
            if i != j {
                ans = ans.min(task[i].0.max(task[j].1));
            } else {
                ans = ans.min(task[i].0 + task[i].1);
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
