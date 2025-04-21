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
        n:usize,a:[usize;n]
    }
    let mut row = vec![(a[0], 1)];
    let mut sum = 1;
    println!("1");
    for i in 1..n {
        let ball = a[i];
        if let Some((b, num)) = row.last_mut() {
            if *b == ball {
                *num += 1;
                sum += 1;
                if num == b {
                    sum -= *num;
                    row.pop();
                }
            } else {
                sum += 1;
                row.push((ball, 1));
            }
        } else {
            row.push((ball, 1));
            sum += 1;
        }
        println!("{}", sum);
    }
}
fn main() {
    solve();
}
