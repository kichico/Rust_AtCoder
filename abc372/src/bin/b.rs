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
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        m:u32
    }
    let mut sum = 0;
    let mut ans: Vec<u32> = Vec::new();
    for i in 0..20 {
        let mut ok = true;
        for a in 0..=10 {
            let t = 3.pow(a);
            if sum + t > m {
                sum += t / 3;
                ok = false;
                ans.push(a - 1);
                break;
            }
        }
        if ok {
            sum += 3.pow(10);
            ans.push(10);
        }
        if sum == m {
            println!("{}", i + 1);
            println!("{}", ans.iter().join(" "));
            return;
        }
    }
}
fn main() {
    solve();
}
