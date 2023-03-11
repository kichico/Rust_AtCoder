#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use regex::internal::Char;
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
        s:Chars
    }
    let n = s.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut hako: HashSet<char> = HashSet::new();
    let mut from_to_end = (0, n);
    for i in 0..n {
        if s[i] == '(' {
            stack.push(i);
        } else if s[i] == ')' {
            let from = *stack.iter().next_back().unwrap();
            let mut end = i.clone();
            while end != from {
                if hako.contains(&s[end]) {
                    hako.remove(&s[end]);
                }
                end -= 1;
                if end == from_to_end.1 {
                    end = from_to_end.0 - 1;
                }
            }
            from_to_end = (from, i.clone());
            stack.pop();
        } else {
            if hako.contains(&s[i]) {
                println!("No");
                return;
            } else {
                hako.insert(s[i]);
            }
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
