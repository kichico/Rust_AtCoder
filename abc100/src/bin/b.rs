#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        d: i64,
        n: i64,
    }
    let ans = if n != 100 {
        100i64.pow(d as u32) * n
    } else {
        100i64.pow(d as u32) * 101
    };
    println!("{}", ans);
}

fn main() {
    solve();
}
