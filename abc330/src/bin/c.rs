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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        d:i64
    }
    let mut ans = 1e18 as i64;
    let mut squares = vec![0, 1i64];
    for i in 2..=1e6 as i64 {
        squares.push(i * i);
    }
    for x in 1..=d.sqrt() {
        let rest = (x * x - d).abs();
        let mut higher = squares.upper_bound(&rest);
        if higher >= squares.len() {
            higher -= 1;
        }
        let lower = higher - 1;
        if rest.abs_diff(squares[higher]) < rest.abs_diff(squares[lower]) {
            ans = ans.min(rest.abs_diff(squares[higher]) as i64)
        } else {
            ans = ans.min(rest.abs_diff(squares[lower]) as i64)
        };
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
