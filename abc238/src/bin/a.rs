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
        n: u32,
    }
    if n > 60 {
        println!("Yes");
    } else {
        let ans = if 2i64.pow(n) > n.pow(2) as i64 {
            "Yes"
        } else {
            "No"
        };
        println!("{}", ans);
    }
}

fn main() {
    solve();
}
