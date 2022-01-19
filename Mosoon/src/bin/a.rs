#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn solve() {
    input! {
        a: u64,
        b: u64,
    }
    let mut ans = 0;
    for i in 1..=a {
        for j in 1..=b {
            ans += 100 * i + j;
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
