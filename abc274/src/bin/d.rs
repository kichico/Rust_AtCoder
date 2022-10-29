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
        n:usize,x:i64,y:i64,a:[i64;n]
    }
    let mut tate: Vec<i64> = Vec::new();
    let mut yoko: Vec<i64> = Vec::new();
    for i in 0..n {
        if i % 2 == 0 {
            yoko.push(a[i]);
        } else {
            tate.push(a[i]);
        }
    }
    let mut status: HashSet<i64> = HashSet::new();
    status.insert(a[0]);
    for i in 1..yoko.len() {
        let mut next_status: HashSet<i64> = HashSet::new();
        for nx in &status {
            next_status.insert(*nx + yoko[i]);
            next_status.insert(*nx - yoko[i]);
        }
        status = next_status;
    }
    if !status.contains(&x) {
        println!("No");
        return;
    }
    let mut status: HashSet<i64> = HashSet::new();
    status.insert(0);
    for i in 0..tate.len() {
        let mut next_status: HashSet<i64> = HashSet::new();
        for nx in &status {
            next_status.insert(*nx + tate[i]);
            next_status.insert(*nx - tate[i]);
        }
        status = next_status;
    }
    if !status.contains(&y) {
        println!("No");
        return;
    }
    println!("Yes");
}

fn main() {
    solve();
}
