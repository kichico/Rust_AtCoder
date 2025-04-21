#[allow(unused_imports)]
use itertools::*;
use maplit::{btreeset, hashset};
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
        n:usize,slot:[Chars;3]
    }
    let mut ans = 1e18 as usize;
    'outer: for i in 0..10 {
        let c = to_char(i);
        for j in 0..3 {
            if !slot[j].contains(&c) {
                continue 'outer;
            }
        }
        let it = (0usize..3usize).into_iter().permutations(3);
        for v in it {
            let mut time: BTreeSet<usize> = BTreeSet::new();
            for idx in &v {
                let mut now = 0usize;
                loop {
                    if slot[*idx][now % n] == c && !time.contains(&now) {
                        time.insert(now);
                        break;
                    }
                    now += 1;
                }
            }
            ans = ans.min(*time.iter().next_back().unwrap());
        }
    }
    if ans == 1e18 as usize {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
fn main() {
    solve();
}
