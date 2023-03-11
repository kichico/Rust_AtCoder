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
        n:usize,q:usize
    }
    let mut card = vec![0; n];
    for i in 0..q {
        input! {x:usize,p:Usize1}
        if x == 1 {
            card[p] += 1;
        } else if x == 2 {
            card[p] += 2;
        } else {
            let ans = if card[p] >= 2 { "Yes" } else { "No" };
            println!("{}", ans);
        }
    }
}
fn main() {
    solve();
}
