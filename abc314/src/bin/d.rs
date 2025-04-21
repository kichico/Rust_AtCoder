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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut s:Chars,q:usize,query:[(usize,usize,char);q]
    }
    let mut last_move = (4, 0);
    for i in 0..q {
        let (que, pos, c) = query[i];
        if que == 1 {
            s[pos - 1] = c;
        } else {
            last_move = (que, i);
        }
    }
    if last_move.0 == 4 {
        println!("{}", s.iter().join(""));
        return;
    }
    if last_move.0 == 2 {
        for i in 0..n {
            s[i] = s[i].to_ascii_lowercase();
        }
    } else {
        for i in 0..n {
            s[i] = s[i].to_ascii_uppercase();
        }
    }
    for i in last_move.1 + 1..q {
        let (_que, pos, c) = query[i];
        s[pos - 1] = c;
    }
    println!("{}", s.iter().join(""));
}
fn main() {
    solve();
}
