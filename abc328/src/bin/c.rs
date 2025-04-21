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
        n:usize,q:usize,s:Chars,query:[(Usize1,Usize1);q]
    }
    let mut ans = vec![0; q];
    let mut dup: Vec<usize> = Vec::new();
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dup.push(i);
        }
    }
    for i in 0..q {
        let (l, r) = query[i];
        if l == r {
            ans[i] = 0;
        } else {
            let right = dup.lower_bound(&r);
            let left = dup.lower_bound(&l);
            ans[i] = right - left;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
