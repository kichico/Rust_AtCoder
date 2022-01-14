#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve() {
    input! {
        n: usize,
        t:[(i64,i64);n],
    }
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let (xa, ya) = t[i];
                let (xb, yb) = t[j];
                let (xc, yc) = t[k];
                //dbg!(i, j, k);
                //dbg!(xb - xa, xc - xa, yb - ya, yc - ya);
                if (xb - xa) * (yc - ya) == (xc - xa) * (yb - ya) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

fn main() {
    solve();
}
