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
        n:usize,a:i64,b:i64,s:Chars
    }
    let mut deq: VecDeque<char> = VecDeque::new();
    for c in s {
        deq.push_back(c);
    }
    let mut ans = 1e18 as i64;
    let mut cost = 0;
    for i in 0..n / 2 {
        if deq[i] != deq[n - i - 1] {
            cost += b;
        }
    }
    ans = ans.min(cost);
    for rotate in 1..=n {
        let v = deq.pop_front().unwrap().clone();
        deq.push_back(v);
        let mut cost = 0;
        for i in 0..n / 2 {
            if deq[i] != deq[n - i - 1] {
                cost += b;
            }
        }
        ans = ans.min(a * rotate as i64 + cost);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
