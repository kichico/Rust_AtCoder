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
        n:usize,mut a:[[i64;n];n],b:[[i64;n];n]
    }
    'outer: for _k in 0..4 {
        let mut rot = vec![vec![-1; n]; n];
        for i in 0..n {
            for j in 0..n {
                rot[n - j - 1][i] = a[i][j];
            }
        }
        a = rot;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    continue 'outer;
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
fn main() {
    solve();
}
