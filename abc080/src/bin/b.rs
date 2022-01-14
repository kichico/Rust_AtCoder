#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: i64,
    }
    let mut sum: i64 = 0;
    for c in n.to_string().as_str().chars() {
        sum += c as i64 - '0' as i64;
    }
    let ans = if n % sum == 0 { "Yes" } else { "No" };
    println!("{}", ans);
}

fn main() {
    solve();
}
