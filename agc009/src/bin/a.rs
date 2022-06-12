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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        x:[(i64,i64);n]
    }
    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    for (i, j) in x {
        a.push(i);
        b.push(j);
    }
    let mut ans = 0;
    for i in (0..n).rev() {
        if a[i] % b[i] == 0 {
            if i > 0 {
                a[i - 1] += ans;
            }
            continue;
        }
        let value = b[i] - (a[i] % b[i]);
        if value > 1 {
            ans += value;
        }
        if i > 0 {
            a[i - 1] += ans;
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
