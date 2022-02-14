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
        n: i64,
    }
    let lower = i32::MIN as i64;
    let upper = i32::MAX as i64;
    dbg!(&lower, &upper);
    let ans = if lower <= n && n <= upper {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn main() {
    solve();
}
