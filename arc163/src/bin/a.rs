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
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        t:usize
    }
    for _ in 0..t {
        input! { n:usize,mut s:Chars }
        let mut ss = vec![0; n];
        for i in 0..n {
            ss[i] = s[i] as i64 - 'a' as i64 + 1;
        }
        let mut vals: Vec<i64> = Vec::new();
        let mut val = 0;
        for i in 0..n {
            if val == 0 {
                val = ss[i];
            } else if ss[i] < ss[i - 1] {
                vals.push(val);
                val = ss[i];
            }else{
                
            }
        }
    }
}
fn main() {
    solve();
}
