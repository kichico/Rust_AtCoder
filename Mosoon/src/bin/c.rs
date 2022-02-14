#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        a: [i64;n],
    }
    let mut ans = 0;
    for left in 0..n {
        let mut minv = a[left];
        for right in left..n {
            minv = min(minv, a[right]);
            ans = max(ans, (right - left + 1) as i64 * minv);
        }
        dbg!(&ans);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
