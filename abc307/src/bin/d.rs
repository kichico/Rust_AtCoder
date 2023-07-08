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
        n:usize,s:Chars
    }
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut left_p = 0;
    for i in 0..n {
        if s[i] == '(' {
            left_p += 1;
            stack.push_back('(');
        } else if s[i] == ')' {
            stack.push_back(')');
            while left_p > 0 && !stack.is_empty() {
                let c = stack.pop_back().unwrap();
                if c == '(' {
                    left_p -= 1;
                    break;
                }
            }
        } else {
            stack.push_back(s[i]);
        }
    }
    if !stack.is_empty() {
        println!("{}", stack.iter().join(""));
    }
}
fn main() {
    solve();
}
