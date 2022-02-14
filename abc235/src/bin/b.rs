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
        n: usize,
        a: [i64;n],
    }
    let mut p = 0;
    let mut r = a[0];
    while p < n - 1 && r < a[p + 1] {
        p += 1;
        r = a[p];
    }
    println!("{}", a[p]);
}

fn main() {
    solve();
}
