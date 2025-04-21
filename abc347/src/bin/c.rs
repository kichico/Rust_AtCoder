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
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:usize,b:usize,plan:[usize;n]
    }
    let plan = plan.iter().map(|x| x % (a + b)).collect::<Vec<usize>>();
    let mut days: Vec<usize> = Vec::new();
    for i in 0..n {
        days.push(plan[i]);
        days.push(plan[i] + a + b);
    }
    days.sort();
    for i in 0..days.len() {
        let v = days.lower_bound(&(days[i] + a)) - i;
        if v == n {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    solve();
}
