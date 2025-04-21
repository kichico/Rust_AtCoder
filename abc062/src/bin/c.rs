#[allow(unused_imports)]
use itertools::*;
use maplit::btreeset;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use pathfinding::prelude::directions::S;
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

fn local(h: usize, w: usize) -> usize {
    let mut candidate: BTreeSet<usize> = BTreeSet::new();
    for hh in 1..h {
        for ww in 1..w {
            let sq = btreeset! {hh*w,(h-hh)*ww,(h-hh)*(w-ww)};
            candidate.insert(*sq.last().unwrap() - *sq.first().unwrap());
            let sq = btreeset! {ww*h,(w-ww)*hh,(h-hh)*(w-ww)};
            candidate.insert(*sq.last().unwrap() - *sq.first().unwrap());
        }
    }
    if h % 3 == 0 || w % 3 == 0 {
        return 0;
    }
    *candidate.first().unwrap()
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        h:usize,w:usize
    }
    let mut candidate: Vec<usize> = Vec::new();
    let mut diff = 1e18 as usize;
    for hh in 1..h {
        let rest = h - hh;
        if w % 2 == 0 {
            let square = btreeset! {hh * w, rest * (w / 2)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        } else {
            let square = btreeset! {hh * w, rest * (w / 2), rest * ((w / 2) + 1)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        }
        if rest % 2 == 0 {
            let square = btreeset! {hh * w, w * (rest / 2)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        } else {
            let square = btreeset! {hh * w, w * (rest / 2), w * ((rest / 2) + 1)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        }
    }
    candidate.push(diff);
    for ww in 1..w {
        let rest = w - ww;
        if h % 2 == 0 {
            let square = btreeset! {h * ww, rest * (h / 2)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        } else {
            let rest = w - ww;
            let square = btreeset! {h * ww, rest * (h / 2), rest * ((h / 2) + 1)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        }
        if rest % 2 == 0 {
            let square = btreeset! {ww * h, h * (rest / 2)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        } else {
            let square = btreeset! {h * ww, h * (rest / 2), h * ((rest / 2) + 1)};
            let biggest = square.last().unwrap();
            let smallest = square.first().unwrap();
            diff = diff.min(biggest - smallest);
        }
    }
    candidate.push(diff);
    candidate.sort();
    println!("{}", candidate[0]);
}
fn main() {
    solve();
}
