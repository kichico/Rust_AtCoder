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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,s:Chars
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..s.len() {
        if s[i] == 'L' {
            que.push_front(i + 1);
        } else if s[i] == 'R' {
            que.push_back(i + 1);
        } else if s[i] == 'A' {
            if que.is_empty() {
                println!("ERROR");
            } else {
                let c = que.pop_front();
                println!("{}", c.unwrap());
            }
        } else if s[i] == 'B' {
            if que.len() <= 1 {
                println!("ERROR");
            } else {
                let f = que.pop_front().unwrap();
                let s = que.pop_front().unwrap();
                println!("{}", s);
                que.push_front(f);
            }
        } else if s[i] == 'C' {
            if que.len() <= 2 {
                println!("ERROR");
            } else {
                let f = que.pop_front().unwrap();
                let s = que.pop_front().unwrap();
                let t = que.pop_front().unwrap();
                println!("{}", t);
                que.push_front(s);
                que.push_front(f);
            }
        } else if s[i] == 'D' {
            if que.is_empty() {
                println!("ERROR");
            } else {
                let c = que.pop_back();
                println!("{}", c.unwrap());
            }
        } else if s[i] == 'E' {
            if que.len() <= 1 {
                println!("ERROR");
            } else {
                let f = que.pop_back().unwrap();
                let s = que.pop_back().unwrap();
                println!("{}", s);
                que.push_back(f);
            }
        } else if s[i] == 'F' {
            if que.len() <= 2 {
                println!("ERROR");
            } else {
                let f = que.pop_back().unwrap();
                let s = que.pop_back().unwrap();
                let t = que.pop_back().unwrap();
                println!("{}", t);
                que.push_back(s);
                que.push_back(f);
            }
        }
    }
}
fn main() {
    solve();
}
