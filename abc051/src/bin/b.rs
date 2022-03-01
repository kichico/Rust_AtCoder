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
        K: i64,
        S: i64,
    }
    let mut cnt = 0;
    for i in 0..=K {
        for j in 0..=K {
            let k = S - i - j;
            if k >= 0 && k <= K {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
