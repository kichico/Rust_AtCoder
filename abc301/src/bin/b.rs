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
        n:usize,a:[i64;n]
    }
    let mut ans: Vec<i64> = vec![a[0]];
    for i in 1..n {
        let from = a[i - 1];
        let to = a[i];
        if abs(from - to) <= 1 {
            ans.push(to);
            continue;
        }
        if from <= to {
            for x in from + 1..=to {
                ans.push(x);
            }
        } else {
            for x in (to..from).rev() {
                ans.push(x);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
fn main() {
    solve();
}
