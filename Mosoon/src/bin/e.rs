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
        s: String,
    }
    let v = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    for i in 0..7 {
        if s == v[i].to_string() {
            println!("{}", 7 - i);
            return;
        }
    }
}

fn main() {
    solve();
}
