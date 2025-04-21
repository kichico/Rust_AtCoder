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
        s:Chars
    }
    let lane = vec![3, 2, 4, 1, 3, 5, 0, 2, 4, 6];
    let mut standing: Vec<Vec<usize>> = vec![Vec::new(); 7];
    for i in 0..s.len() {
        if s[i] == '1' {
            standing[lane[i]].push(i);
        }
    }
    if s[0] != '0' {
        println!("No");
        return;
    }
    for i in 0..5 {
        for j in i + 2..7 {
            for k in i + 1..j {
                if !standing[i].is_empty() && !standing[j].is_empty() && standing[k].is_empty() {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
