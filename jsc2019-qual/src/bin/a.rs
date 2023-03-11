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
        M: i64,
        D: i64,
    }
    let mut cnt = 0;
    for m in 1..=M {
        for i in 2..10 {
            for j in 2..10 {
                let date = i * 10 + j;
                if date > D {
                    continue;
                }
                if i * j == m {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
