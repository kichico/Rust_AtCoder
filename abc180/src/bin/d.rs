#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        mut p: u128,
        limit: u128,
        bai: u128,
        plus: u128,
    }
    let mut cnt: u128 = 0;
    while p * bai <= plus && p * bai < limit {
        p *= bai;
        cnt += 1;
    }
    dbg!(limit, p, limit - p);
    let v = if limit == p { 0 } else { limit - p - 1 };
    cnt += (v) / plus;
    println!("{}", cnt);
}

fn main() {
    solve();
}
