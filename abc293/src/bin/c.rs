#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
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
use std::iter::FromIterator;
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
        h:usize,w:usize,area:[[i64;w];h]
    }
    let mut dir: Vec<usize> = Vec::new();
    let it = (0..h + w - 2).into_iter();
    let mut ans = 0;
    for v in it.combinations(h - 1) {
        let mut pos = 0;
        let mut num = btreeset! {area[0][0]};
        let mut x = 0;
        let mut y = 0;
        for i in 0..h + w - 2 {
            if pos < v.len() && i == v[pos] {
                y += 1;
                pos += 1;
            } else {
                x += 1;
            }
            num.insert(area[y][x]);
        }
        if num.len() == h + w - 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
