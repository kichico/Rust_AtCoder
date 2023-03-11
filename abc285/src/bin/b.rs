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
        n:usize,s:Chars
    }
    let mut ans = vec![0; n - 1];
    for step in 1..n {
        let mut now = 0;
        while now + step < n {
            if s[now] == s[now + step] {
                ans[step - 1] = now;
                break;
            } else {
                now += 1;
            }
        }
        if now == n - step {
            ans[step - 1] = now;
        }
    }
    for i in ans {
        println!("{}", i);
    }
}
fn main() {
    solve();
}
