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
        n:usize,m:usize,somen:[(i64,i64,i64);m]
    }
    let mut timetable: BinaryHeap<(Reverse<i64>, usize, usize, usize)> = BinaryHeap::new();
    for i in 0..m {
        timetable.push((Reverse(somen[i].0), 0, i, n + 1));
    }
    let mut people = BTreeSet::from_iter((0..n).into_iter());
    let mut ans = vec![0; n];
    while let Some((time, iscomeback, id, comebacker)) = timetable.pop() {
        if iscomeback == 0 {
            if people.is_empty() {
                continue;
            }
            let p = *people.iter().next().unwrap();
            //println!("get:{} time:{} amount:{}", p + 1, time.0, somen[id].1);
            ans[p] += somen[id].1;
            people.remove(&p);
            timetable.push((Reverse(somen[id].0 + somen[id].2), 1, id, p));
        } else {
            people.insert(comebacker);
            //println!("comeback:{} time:{}", comebacker + 1, time.0);
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
