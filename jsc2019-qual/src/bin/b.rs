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
fn solve() {
    input! {
        n: usize,
        k: i128,
        a: [i128;n],
    }
    const MOD: i128 = 1e9 as i128 + 7;
    let mut cnt_after = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if a[i] > a[j] {
                cnt_after += 1;
                cnt_after %= MOD;
            }
        }
    }
    let mut cnt_before = 0;
    let mut b = a.clone();
    b.append(&mut a.clone());
    for i in 1..n {
        for j in 0..i {
            if b[i] > b[n + j] {
                cnt_before += 1;
                cnt_before %= MOD;
            }
        }
    }
    println!(
        "{}",
        ((cnt_after * ((k + 1) * k / 2)) % MOD + (cnt_before * (k - 1) * k / 2) % MOD) % MOD
    );
}

fn main() {
    solve();
}
