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
        n:usize,k:usize,mut a:[usize;n]
    }
    let mut ans = a.clone();
    ans.sort();
    let mut each_k: Vec<Vec<usize>> = vec![Vec::new(); k];
    for i in 0..n {
        each_k[i % k].push(a[i]);
    }
    for i in 0..k {
        each_k[i].sort();
    }
    let size = each_k[each_k.len() - 1].len();
    let mut constructed: Vec<usize> = Vec::new();
    for i in 0..size {
        for row in 0..k {
            constructed.push(each_k[row][i]);
        }
    }
    for i in 0..k {
        if size != each_k[i].len() {
            constructed.push(*each_k[i].iter().last().unwrap());
        }
    }
    if constructed != ans {
        println!("No");
    } else {
        println!("Yes");
    }
}
fn main() {
    solve();
}
