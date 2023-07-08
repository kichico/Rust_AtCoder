#[allow(unused_imports)]
use itertools::*;
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
        n:usize,_m:usize,ss:[Chars;n]
    }
    'outer: for v in (0..n).into_iter().permutations(n) {
        for p in 1..n {
            let i = v[p - 1];
            let j = v[p];
            let mut diff = 0;
            for (s, t) in zip(&ss[i], &ss[j]) {
                if s != t {
                    diff += 1;
                }
            }
            if diff > 1 {
                continue 'outer;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
fn main() {
    solve();
}
