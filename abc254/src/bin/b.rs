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
        n:usize
    }
    let mut prev = vec![1];
    println!("{}", prev.iter().join(" "));
    for i in 1..n {
        let mut current = vec![1; i + 1];
        for j in 0..(i + 1) {
            if j == 0 || j == i {
                continue;
            }
            current[j] = prev[j - 1] + prev[j];
        }
        println!("{}", &current.iter().join(" "));
        prev = current;
    }
}
fn main() {
    solve();
}
