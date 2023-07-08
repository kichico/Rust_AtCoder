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
        t:usize
    }
    let mut ans = vec![0; t];
    for k in 0..t {
        input! { n:usize,mut p:[usize;n] }
        let mut cnt = 0;
        'outer: for i in 0..n {
            for j in i + 1..n {
                if p[i] > p[j] {
                    continue 'outer;
                }
            }
            cnt += 1;
        }
        ans[k] = cnt;
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
