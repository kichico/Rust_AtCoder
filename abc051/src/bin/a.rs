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
        s: String,
    }
    let mut ans: Vec<char> = Vec::new();
    let v: Vec<char> = s.as_str().chars().collect();
    let mut a: Vec<String> = Vec::new();
    for c in v {
        if c != ',' {
            ans.push(c);
        } else {
            let ss: String = ans.iter().collect();
            a.push(ss);
            ans.clear();
        }
    }
    let ss: String = ans.iter().collect();
    a.push(ss);
    println!("{} {} {}", &a[0], &a[1], &a[2]);
}

fn main() {
    solve();
}
