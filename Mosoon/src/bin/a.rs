#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn solve() {
    input! {
        vv: String,
    }
    let v: Vec<char> = vv.as_str().chars().collect();
    let mut cnt = 0;
    for x in v {
        if x == 'o' {
            cnt += 1;
        }
    }
    println!("{}", 700 + cnt * 100);
}

fn main() {
    solve();
}
