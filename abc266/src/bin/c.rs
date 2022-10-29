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
        rect:[(i64,i64);4]
    }
    let mut ang = vec![0; 4];
    for i in 0..4 {
        let (x, y) = rect[i];
        let mut sm = i - 1;
        if sm < 0 {
            sm += 4;
        }
        let mut la = i + 1;
        if la >= 4 {
            la -= 4;
        }
        let (sx, sy) = rect[sm as usize];
        let (lx, ly) = rect[la as usize];
        let vs = (sx - x, sy - y);
        let vl = (lx - x, ly - y);
        let innerproduct = vs.0 * vl.0 + vs.1 * vl.1;
        let theta = (innerproduct / (vs.0.pow(2) + vs.1.pow(2).sqrt())
            * (vl.0.pow(2) + vl.1.pow(2).sqrt())) as f64;
        let theta = theta.cos().acos();
    }
}

fn main() {
    solve();
}
