#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,MOD:i64,mut a:[i64;n]
    }
    let mut org = a.clone();
    for i in 0..n {
        a[i] %= MOD;
    }
    let mut all = vec![(0, 0); n];
    for i in 0..n {
        all[i] = (a[i], i);
    }
    all.sort();
    let mut v = a.clone();
    for i in 0..n {
        v.push(a[i]);
    }
    let mut longest = 0;
    let mut ans = (0, 0);
    let mut left = 0;
    let mut right = 1;
    if n == 1 {
        println!("0");
        return;
    }
    loop {
        if right >= n {
            break;
        }
        let mut last_value = v[left].clone();
        right = right.max(left + 1);
        while right < 2 * n && ((last_value + 1) % MOD == v[right] || last_value == v[right]) {
            //println!("last value :{} {}", last_value, v[right + 1]);
            last_value = v[right.clone()];
            right += 1;
            //println!("last value :{} {}", last_value, v[right]);
        }
        //println!("left : {} right : {}", ans.0, ans.1);
        left += 1;
    }
    org.sort();
}
fn main() {
    solve();
}
