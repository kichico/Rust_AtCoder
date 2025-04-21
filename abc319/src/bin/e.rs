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
        n:usize,x:usize,y:usize,daiya:[(usize,usize);n-1],q:usize,query:[usize;q]
    }
    let mut ans = vec![0; 10];
    for p in 0..=9 {
        let mut current = p;
        for (depart, elapse) in &daiya {
            let wait = (depart - current % depart) % depart;
            current += wait + elapse;
        }
        current += y;
        ans[p] = current;
    }
    for time in query {
        let t = (time + x) % daiya[0].0;
        println!("{}", time + x + ans[t]);
    }
}
fn main() {
    solve();
}
