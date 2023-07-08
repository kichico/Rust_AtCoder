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
        s:Chars
    }
    let mut r: Vec<usize> = Vec::new();
    for i in 0..s.len() {
        if s[i] == 'B' {
            r.push(i);
        }
    }
    if r[0] % 2 == r[1] % 2 {
        println!("No");
        return;
    }
    r.clear();
    let mut k = 0;
    for i in 0..s.len() {
        if s[i] == 'R' {
            r.push(i);
        }
        if s[i] == 'K' {
            k = i;
        }
    }
    if r[0] < k && k < r[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn main() {
    solve();
}
