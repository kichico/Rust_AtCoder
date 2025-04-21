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
        mut h:usize,mut w:usize,field:[Chars;h]
    }

    let mut ans = vec![vec!['#'; w + 2]; h + 2];

    for i in 0..h {
        for j in 0..w {
            ans[i + 1][j + 1] = field[i][j];
        }
    }
    for i in 0..h + 2 {
        println!("{}", ans[i].iter().join(""));
    }
}
fn main() {
    solve();
}
