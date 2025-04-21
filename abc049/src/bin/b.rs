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
        h:usize,w:usize,img:[Chars;h]
    }
    let mut resized = vec![vec!['.'; w]; 2 * h];
    for i in 0..2 * h {
        for j in 0..w {
            resized[i][j] = img[i / 2][j];
        }
    }
    for i in 0..2 * h {
        println!("{}", resized[i].iter().join(""));
    }
}
fn main() {
    solve();
}
