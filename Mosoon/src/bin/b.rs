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
        N: i64,
        a: i64,
        b: i64,
    }
    let bb = if N <= 5 { N * b } else { 5 * b };
    let aa = max(N - 5, 0);
    println!("{}", bb + aa * a);
}

fn main() {
    solve();
}