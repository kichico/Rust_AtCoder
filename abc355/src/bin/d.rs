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
        n:usize,kukan:[(usize,usize);n]
    }
    let mut asshuku = BTreeSet::new();
    let mut dic = BTreeMap::new();
    for (l, r) in &kukan {
        asshuku.insert(*l);
        asshuku.insert(*r);
    }
    for (i, val) in enumerate(&asshuku) {
        dic.insert(*val, i);
    }
    let kukan = kukan
        .iter()
        .map(|(x, y)| (*dic.get(x).unwrap(), *dic.get(y).unwrap()))
        .collect::<Vec<_>>();
    //println!("{}", asshuku.iter().join(" "));
    let mut dubling = HashSet::new();
    let mut sum_kukan = Vec::new();
    for i in 0..kukan.len() {
        let (l, r) = kukan[i];
        sum_kukan.push((l, 0, i));
        sum_kukan.push((r, 1, i));
    }
    sum_kukan.sort();
    let mut ans = 0;
    for i in 0..sum_kukan.len() {
        let (_pos, _ord, id) = sum_kukan[i];
        if !dubling.contains(&id) {
            if dubling.len() >= 1 {
                ans += dubling.len();
            }
            dubling.insert(id);
        } else {
            dubling.remove(&id);
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
