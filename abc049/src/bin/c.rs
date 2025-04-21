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
        mut s:Chars
    }
    let append = vec![
        "dream".chars().collect::<Vec<char>>(),
        "dreamer".chars().collect::<Vec<char>>(),
        "erase".chars().collect::<Vec<char>>(),
        "eraser".chars().collect::<Vec<char>>(),
    ];
    let mut avail = true;
    while avail {
        avail = false;
        for t in &append {
            if s.len() < t.len() {
                continue;
            }
            //dbg!(&s[s.len() - t.len()..s.len()], &t[0..t.len()]);
            if s[s.len() - t.len()..s.len()] == t[0..t.len()] {
                avail = true;
                for i in 0..t.len() {
                    s.pop();
                }
                break;
            }
        }
    }
    if s.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
fn main() {
    solve();
}
