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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

fn dfs(a: Vec<usize>) -> usize {
    let mut current_ans = Vec::new();
    if a.len() == 3 {
        let mut one = 0;
        let mut zero = 0;
        for i in 0..3 {
            if a[i] == 0 {
                zero += 1;
            } else {
                one += 1;
            }
        }
        if one > zero {
            return one;
        }
        return zero;
    }
    for i in (0..a.len()).step_by(3) {
        let mut v: Vec<usize> = Vec::new();
        for id in 0..3 {
            v.push(i + id);
        }
        current_ans.push(dfs(v));
    }
    let mut one = 0;
    let mut zero = 0;
    for i in 0..3 {
        if current_ans[i] == 0 {
            zero += 1;
        } else {
            one += 1;
        }
    }
    if one > zero {
        return one;
    }
    return zero;
}

#[allow(non_snake_case)]
fn to_Binary(mut x: usize) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    if x == 0 {
        v.push('0');
    }
    while x > 0 {
        let c = std::char::from_digit((x % 2) as u32, 10).unwrap();
        v.push(c);
        x /= 2;
    }
    v.reverse();
    return v;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:u32,a:Chars
    }
    let a = a
        .iter()
        .map(|x| (*x as u32 - '0' as u32) as usize)
        .collect_vec();
    let default_ans = dfs(a);
    for i in 0..2.pow(n) {
        let v = to_Binary(i)
            .iter()
            .map(|x| (*x as u32 - '0' as u32) as usize)
            .collect_vec();
        if default_ans
    }
}
fn main() {
    solve();
}
