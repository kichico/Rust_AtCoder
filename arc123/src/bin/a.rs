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
        a: [i64;3],
    }
    let mut ans = 0;
    let diff1 = a[1] - a[0];
    let diff2 = a[2] - a[1];
    if diff1 == diff2 {
        println!("0");
        return;
    }
    if diff1 < 0 && diff2 > 0 {
        let ave = (a[0] + a[2]) as f64 / 2 as f64;
        ans = if ave % 2 == 0 {
            ave as i64 - a[1]
        } else {
            ave - a[1] + 1
        };
    }
}

fn main() {
    solve();
}
