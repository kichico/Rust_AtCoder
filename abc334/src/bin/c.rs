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
        n:usize,k:usize,mut lost:[usize;k]
    }
    let mut ans = 0;
    if k % 2 == 0 {
        for i in (0..k).into_iter().step_by(2) {
            ans += lost[i + 1] - lost[i];
        }
    } else {
        if k == 1 {
            println!("0");
            return;
        }
        let mut current_total = 0;
        for i in (0..k - 1).into_iter().step_by(2) {
            println!("k:{}", i);
            current_total += lost[i + 1] - lost[i];
        }
        ans = current_total.clone();
        for i in (2..k).rev() {
            current_total -= lost[i - 1] - lost[i - 2];
            current_total += lost[i] - lost[i - 2];
            ans = ans.min(current_total);
            println!("{}:{}", i, current_total);
        }
        current_total = 0;
        for i in (1..k).into_iter().step_by(2) {
            current_total += lost[i + 1] - lost[i];
        }
        ans = ans.min(current_total);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
