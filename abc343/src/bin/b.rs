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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mat:[[usize;n];n]
    }
    let mut g = vec![BTreeSet::new(); n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if mat[i][j] == 1 {
                g[i].insert(j + 1);
                g[j].insert(i + 1);
            }
        }
    }
    for i in 0..n {
        println!("{}", g[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
