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
        n:usize,t:usize,a:[Usize1;t]
    }
    let mut tate = vec![0; n];
    let mut yoko = vec![0; n];
    let mut naname = vec![0; 2];
    for (i, v) in enumerate(a) {
        let row = v / n;
        let col = v % n;
        tate[row] += 1;
        yoko[col] += 1;
        if row == col {
            naname[0] += 1;
        }
        if row + col == n - 1 {
            naname[1] += 1;
        }
        if tate[row] == n || yoko[col] == n || naname[0] == n || naname[1] == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
fn main() {
    solve();
}
