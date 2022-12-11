#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,k:usize,mut a:[i64;n]
    }
    let mut other = a.clone();
    other.sort();
    let mut group: HashMap<usize, Vec<i64>> = HashMap::new();
    for i in 0..n {
        let e = group.entry(i % k).or_insert(Vec::new());
        e.push(a[i % k]);
    }
    for (_i, v) in &mut group {
        v.sort();
        v.reverse();
    }
    let mut ans: Vec<i64> = Vec::new();
    for i in 0..n {
        let v = group.get_mut(&(i % k)).unwrap();
        ans.push(v.iter().next_back().unwrap().clone());
        v.pop();
    }
    for i in 0..n {
        if other[i] != ans[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
