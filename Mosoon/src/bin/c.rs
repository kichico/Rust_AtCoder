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
        s:String,
        n: usize,
    }
    let vc: Vec<char> = s.chars().collect();
    let mut st: Vec<String> = Vec::new();
    for x in 0..5 {
        for y in 0..5 {
            st.push([vc[x].to_string(), vc[y].to_string()].concat());
        }
    }
    st.sort();
    println!("{}", st[n - 1]);
}

fn main() {
    solve();
}
