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

fn run_length_compression<T>(v: Vec<T>) -> Vec<(T, usize)>
where
    T: Clone + Copy + PartialEq,
{
    let mut ret: Vec<(T, usize)> = Vec::new();
    let mut last_element = v[0];
    let mut len = 1;
    for i in 1..v.len() {
        if v[i] == last_element {
            len += 1;
        } else {
            ret.push((last_element, len));
            last_element = v[i];
            len = 1;
        }
    }
    ret.push((last_element, len));
    return ret;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[i64;n]
    }
    if n == 1 {
        println!("1");
        return;
    }
    let kaisa = a
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect::<Vec<i64>>();
    let rlc = run_length_compression(kaisa);
    let ans = rlc
        .iter()
        .map(|(_x, num)| (num + 1) * num / 2)
        .sum::<usize>()
        + n;
    println!("{}", ans);
}
fn main() {
    solve();
}
