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
        h:usize,w:usize,field:[Chars;h]
    }
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();

    for r in 0..h {
        for c in 0..w {
            if field[r][c] == '#' {
                xs.insert(c);
                ys.insert(r);
            }
        }
    }
    let xmin = xs.first().unwrap();
    let xmax = xs.last().unwrap();
    let ymin = ys.first().unwrap();
    let ymax = ys.last().unwrap();
    for r in *ymin..=*ymax {
        for c in *xmin..=*xmax {
            if field[r][c] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
fn main() {
    solve();
}
