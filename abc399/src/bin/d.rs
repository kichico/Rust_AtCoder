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
        t:usize
    }
    let mut ans = vec![0; t];
    for tt in 0..t {
        input! { n:usize, a:[usize;n*2] }
        let mut postion = vec![Vec::new(); n + 1];
        for i in 0..2 * n {
            postion[a[i]].push(i);
        }
        let mut answers = BTreeSet::new();
        for i in 0..2 * n - 1 {
            let an = a[i];
            let bn = a[i + 1];
            if postion[an][0] + 1 == postion[an][1] || postion[bn][0] + 1 == postion[bn][1] {
                continue;
            }
            let mut v = vec![
                postion[an][0],
                postion[an][1],
                postion[bn][0],
                postion[bn][1],
            ];
            v.sort();
            if v[0] + 1 == v[1] && v[2] + 1 == v[3] {
                answers.insert((v[0], v[3]));
            }
            ans[tt] = answers.len();
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
