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
        n:usize,s:[String;n]
    }
    for i in 0..n {
        'outer: for j in 0..n {
            if i == j {
                continue;
            }
            let b: &str = &s[j];
            let ss = s[i].clone() + b;
            let ss: Vec<char> = ss.chars().collect();
            for k in 0..ss.len() / 2 + 1 {
                if ss[k] != ss[ss.len() - k - 1] {
                    continue 'outer;
                }
            }
            println!("Yes");
            return;
        }
    }
    println!("No");
}
fn main() {
    solve();
}
