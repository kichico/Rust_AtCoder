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
        n:usize
    }
    let mut kake_num: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut kake: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..n {
        input! {
            c:usize,a:[usize;c]
        }
        kake[i] = HashSet::from_iter(a.into_iter());
    }
    input! {x:usize}
    for i in 0..n {
        if kake[i].contains(&x) {
            kake_num.insert((kake[i].len(), i + 1));
        }
    }
    if kake_num.is_empty() {
        println!("0");
        println!();
        return;
    }
    let mini = kake_num.iter().next().unwrap().0;
    let mut ans: Vec<usize> = Vec::new();
    for (num, id) in kake_num {
        if num == mini {
            ans.push(id);
        }
    }
    ans.sort();
    println!("{}", ans.len());
    if ans.len() != 0 {
        println!("{}", ans.iter().join(" "));
    }
}
fn main() {
    solve();
}
