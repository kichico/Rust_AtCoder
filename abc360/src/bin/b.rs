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
        s:Chars,t:String
    }
    for w in 1..s.len() {
        let mut total = Vec::new();
        for it in (0..s.len()).step_by(w) {
            let mut tp = Vec::new();
            for i in 0..w {
                if it + i >= s.len() {
                    break;
                }
                tp.push(s[it + i]);
            }
            total.push(tp);
        }

        for c in 0..w {
            let mut tp = Vec::new();
            for i in 0..total.len() {
                if total[i].len() <= c {
                    continue;
                }
                tp.push(total[i][c]);
            }
            //println!("w:{} tp:{}", w, tp.iter().join(""));
            if tp.iter().collect::<String>() == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
fn main() {
    solve();
}
