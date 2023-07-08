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
        n:usize,k:usize,a:[[i64;n];n],q:usize,query:[(Usize1,Usize1);q]
    }
    let mut path = vec![vec![1e18 as i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                path[i][j] = 1;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                path[i][j] = std::cmp::min(path[i][j], path[i][k] + path[k][j]);
            }
        }
    }
    let mut ans: Vec<i64> = Vec::new();
    for (mut s, mut t) in query {
        s %= n;
        t %= n;
        if path[s][t] == 1e18 as i64 {
            ans.push(-1);
        } else {
            ans.push(path[s][t]);
        }
    }
    for s in ans {
        println!("{}", s);
    }
}
fn main() {
    solve();
}
