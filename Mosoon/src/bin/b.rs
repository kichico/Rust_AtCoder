#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
#[fastout]

fn solve() {
    input! {
        n: usize,
        m: usize,
        t: i64,
        a: [(i64,i64);m],
    }
    let mut current: i64 = n as i64;
    let mut now = 0;
    for (from, to) in a {
        current -= from - now;
        if current <= 0 {
            println!("No");
            return;
        }
        current += to - from;
        if current >= n as i64 {
            current = n as i64;
        }
        now = to;
    }
    if t - now < current {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    solve();
}
