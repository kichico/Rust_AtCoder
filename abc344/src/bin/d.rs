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

#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        t:String,n:usize
    }
    let mut store = Vec::new();
    for i in 0..n {
        input! {
            m:usize,s:[String;m]
        }
        store.push(s);
    }
    let mut dp = BTreeSet::new();
    for i in 0..store[0].len() {
        dp.insert((store[0][i].clone(), 1));
    }
    for i in 1..n {
        let mut nextDP = dp.clone();
        for (ss, num) in &dp {
            for adder in &store[i] {
                let tt = ss.clone() + adder;

                nextDP.insert((tt, num + 1));
            }
        }
        dp = nextDP;
    }
    for (ss, num) in dp {
        if ss == t {
            println!("{}", num);
            return;
        }
    }
    println!("-1");
}
fn main() {
    solve();
}
