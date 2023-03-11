#[allow(unused_imports)]
use itertools::Itertools;
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
        h:usize,w:usize,table:[[i64;w];h]
    }
    let mut ver = vec![0i64; h];
    let mut hori = vec![0i64; w];
    for i in 0..h {
        ver[i] = table[i].iter().sum();
    }
    for j in 0..w {
        let mut sum = 0;
        for i in 0..h {
            sum += table[i][j];
        }
        hori[j] = sum;
    }
    let mut ans = table.clone();
    for i in 0..h {
        for j in 0..w {
            ans[i][j] = hori[j] + ver[i] - table[i][j];
        }
    }
    for i in 0..h {
        println!("{}", ans[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
