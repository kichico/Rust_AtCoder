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
        mut s:Chars,t:Chars
    }
    let mut big2small: Vec<usize> = Vec::new();
    let mut small2big = big2small.clone();
    let n = s.len();
    for i in 0..n {
        if s[i] as i32 > t[i] as i32 {
            big2small.push(i);
        } else if (s[i] as i32) < t[i] as i32 {
            small2big.push(i);
        }
    }
    small2big.reverse();
    let mut ans = Vec::new();
    let mover = vec![big2small, small2big].concat();
    for i in mover {
        s[i] = t[i];
        ans.push(s.clone());
    }
    println!("{}", ans.len());
    for v in ans {
        println!("{}", v.iter().join(""));
    }
}
fn main() {
    solve();
}
