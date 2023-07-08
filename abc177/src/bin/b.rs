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
        s:Chars,t:Chars
    }
    let n = t.len().clone();
    let mut ans = n.clone();
    if s.len() == n {
        let mut cnt = 0;
        for i in 0..n {
            if s[i] == t[i] {
                cnt += 1;
            }
        }
        println!("{}", n - cnt);
        return;
    }
    for i in 0..s.len() - t.len() {
        let ss = &s.iter().as_slice()[i..i + n];
        let mut cnt = n.clone();
        for (u, v) in zip(ss, &t) {
            if u == v {
                cnt -= 1;
            }
        }
        ans = ans.min(cnt);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
