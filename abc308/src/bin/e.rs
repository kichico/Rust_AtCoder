#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
       n:usize,a:[usize;n],s:Chars
    }
    let mut dict: HashMap<char, Vec<Vec<usize>>> = HashMap::new();
    for (i, c) in enumerate(&s) {
        let e = dict.entry(*c).or_insert(vec![Vec::new(); 3]);
        e[a[i]].push(i);
    }
    let mut mex = vec![0; n];
    if dict.len() != 3 {
        println!("0");
        return;
    }
    let x = dict.get(&'X').unwrap();
    let e = dict.get(&'E').unwrap();
    let m = dict.get(&'M').unwrap();
    for i in 0..n {
        if s[i] != 'E' {
            continue;
        }
        for left in 0..3 {
            for right in 0..3 {
                let mut bt = btreeset! {0,1,2,3};
                bt.remove(&left);
                bt.remove(&right);
                let mini = *bt.iter().next().unwrap();
                let p = x[right].upper_bound(&i);
                mex[i] = mini * (x.len() - p);
            }
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        if s[i] != 'M' {
            continue;
        }
        for left in 0..3 {
            for right in 0..3 {
                let mut bt = btreeset! {0,1,2,3};
                bt.remove(&left);
                bt.remove(&right);
                let mini = *bt.iter().next().unwrap();
                let p = e[right].upper_bound(&i);
                ans[i] = mini * (x.len() - p);
            }
        }
    }
}
fn main() {
    solve();
}
