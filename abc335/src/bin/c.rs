#[allow(unused_imports)]
use itertools::*;
use maplit::btreemap;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
use omniswap::take;
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
        n:usize,q:usize
    }
    let mut body: VecDeque<(i64, i64)> = VecDeque::new();
    for i in 0..n as i64 {
        body.push_back((i + 1, 0));
    }
    let dir = btreemap! {'R'=>(1i64,0),'L'=>(-1,0),'U'=>(0,1),'D'=>(0,-1)};
    for _i in 0..q {
        input! { t:usize }
        if t == 1 {
            input! { c:char }
            let mut p = body[0].clone();
            let d = *dir.get(&c).unwrap();
            p.0 += d.0;
            p.1 += d.1;
            body.pop_back();
            body.push_front(p);
        } else {
            input! { nth:Usize1 }
            println!("{} {}", body[nth].0, body[nth].1);
        }
    }
}
fn main() {
    solve();
}
