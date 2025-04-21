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

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,mut pos:[Usize1;m],mut stone:[usize;m]
    }
    let mut ans = 0;
    let mut rest = stone.iter().sum::<usize>();
    let mut available = n - rest;
    let mut now = 0;
    if pos[0] != 0 {
        println!("-1");
    }
    pos.push(n);
    stone.push(0);
    for i in 0..m {
        let dist = pos[pos.upper_bound(&now)] - now;
        if dist > stone[i] {
            println!("-1");
            return;
        }
        let a = stone[i] - 1;
        if dist == 1 {
            stone[i + 1] += stone[i] - 1;
            ans += stone[i] - 1;
        } else {
            ans += (dist - 1) * (2 * a - (dist - 2)) / 2;
        }
        now = pos[pos.upper_bound(&now)];
    }
    if stone.iter().last().unwrap() != &0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
