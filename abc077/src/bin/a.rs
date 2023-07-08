#[allow(unused_imports)]
use itertools::*;
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
        s:[Chars;2]
    }
    let mut s1 = s[1].clone();
    s1.reverse();
    let mut s0 = s[0].clone();
    s0.reverse();
    let t = vec![s1, s0];
    for i in 0..2 {
        for j in 0..3 {
            if s[i][j] != t[i][j] {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
fn main() {
    solve();
}
