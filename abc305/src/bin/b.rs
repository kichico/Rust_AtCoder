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
        p:char,q:char
    }
    let mut p = p as i32 - 'A' as i32;
    let mut q = q as i32 - 'A' as i32;
    if p > q {
        swap(&mut p, &mut q);
    }
    let d = vec![3, 1, 4, 1, 5, 9];
    let mut ans = 0;
    for i in p..q {
        ans += d[i as usize];
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
