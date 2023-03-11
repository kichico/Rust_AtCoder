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
        s:Chars
    }
    let mut cnt = 0;
    let mut now = 0;
    for _i in 0..s.len() {
        if now >= s.len() {
            break;
        }
        if s[now] == '0' && now != s.len() - 1 && s[now + 1] == '0' {
            cnt += 1;
            now += 2;
        } else {
            now += 1;
        }
    }
    println!("{}", s.len() - cnt);
}
fn main() {
    solve();
}
