#[allow(unused_imports)]
use itertools::Itertools;
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
        n:usize,k:usize,a:[i32;n],q:usize,query:[(Usize1,Usize1);q]
    }
    let mut tmp = vec![vec![0; n]; k];
    for j in 0..k {
        tmp[j][0] -= a[0];
    }
    for i in 0..k {
        println!("{}", tmp[i].iter().join(" "));
    }
    for i in 1..n {
        tmp[0][i] -= a[i];
        for j in 1..k {
            let mut cnt = a[i];
            let init = max(0, i as i64 - k as i64);
            for p in init as usize..i {
                cnt += tmp[j][p];
            }
            println!("{}", cnt);
            tmp[j][i] = -1 * cnt;
        }
    }
    for i in 0..k {
        println!("{}", tmp[i].iter().join(" "));
    }
}
fn main() {
    solve();
}
