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
        n:usize,k:Usize1,s:Chars
    }
    let rlc = run_length_compression(s);
    let ks = rlc
        .iter()
        .enumerate()
        .filter(|(idx, (c, num))| c == &'1')
        .collect_vec();
    let src = ks[k].0;
    let tgt = ks[k - 1].0;
    for i in 0..rlc.len() {
        if i == tgt {
            for _ in 0..rlc[tgt].1 {
                print!("{}", rlc[tgt].0);
            }
            for _ in 0..rlc[src].1 {
                print!("{}", rlc[src].0);
            }
        } else if i == src {
            continue;
        } else {
            for _ in 0..rlc[i].1 {
                print!("{}", rlc[i].0);
            }
        }
    }
    println!();
}
fn main() {
    solve();
}
