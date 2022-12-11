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
        n:usize,query:usize
    }
    let mut connection: HashSet<(usize, usize)> = HashSet::new();
    let mut ans: Vec<&str> = Vec::new();
    for i in 0..query {
        input! {t:i64,mut a:usize,mut b:usize}
        if t == 1 {
            connection.insert((a, b));
        } else if t == 2 {
            if connection.contains(&(a, b)) {
                connection.remove(&(a, b));
            }
        } else {
            if connection.contains(&(a, b)) && connection.contains(&(b, a)) {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }
    for i in ans {
        println!("{}", i);
    }
}
fn main() {
    solve();
}
