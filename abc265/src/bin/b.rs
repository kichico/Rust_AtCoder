#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
        n:usize,m:usize,mut time:i64,a:[i64;n-1],bonus:[(Usize1,i64);m]
    }
    let mut dic: BTreeMap<usize, i64> = BTreeMap::new();
    for i in 0..m {
        dic.insert(bonus[i].0, bonus[i].1);
    }
    for i in 0..n - 1 {
        time -= a[i];
        if time <= 0 {
            println!("No");
            return;
        }
        if dic.contains_key(&(i + 1)) {
            time += dic.get(&(i + 1)).unwrap();
        }
    }
    println!("Yes");
}

fn main() {
    solve();
}
