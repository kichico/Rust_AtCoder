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
        n: usize,
        a:[i64;n],
    }
    let mut cnt = 0;
    for i in 0..n {
        let mut v = a[i].clone();
        while v % 2 == 0 {
            cnt += 1;
            v /= 2;
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
