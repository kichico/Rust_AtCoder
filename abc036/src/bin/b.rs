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
fn rot(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; mat.len()]; mat.len()];
    for i in 0..mat.len() {
        for j in 0..mat.len() {
            ret[i][j] = mat[mat.len() - 1 - j][i];
        }
    }
    return ret;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,field:[Chars;n]
    }
    let r = rot(field);
    for i in 0..n {
        println!("{}", r[i].iter().join(""));
    }
}
fn main() {
    solve();
}
