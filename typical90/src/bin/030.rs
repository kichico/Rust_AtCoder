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
        n: usize,K:i64,
    }
    let mut a: Vec<i64> = vec![0; n + 1];
    let mut div: Vec<i64> = vec![0; n + 1];
    for i in 0..n + 1 {
        div[i] = i as i64;
    }
    for i in 2..n {
        if div[i] == 1 {
            continue;
        }
        let mut coe = 1;
        while coe * i <= n {
            a[coe * i] += 1;
            div[coe * i] /= i as i64;
            coe += 1;
        }
        dbg!(i, div[i]);
    }
    let mut cnt = 0;
    for i in 2..n + 1 {
        if a[i] >= K {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
