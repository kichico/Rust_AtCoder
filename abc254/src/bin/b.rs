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
        n:usize
    }
    let mut ans: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    ans[0].push(1);
    for row in 1..=n {
        for col in 0..row {
            if col == 0 || col == row - 1 {
                ans[row].push(1);
            } else {
                let c = ans[row - 1][col - 1].clone() + ans[row - 1][col].clone();
                ans[row].push(c);
            }
        }
    }
    for i in 1..=n {
        println!("{}", ans[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
