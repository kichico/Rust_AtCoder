#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
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
        n:usize,
        a:[String;n],
    }
    let mut st: HashMap<String, i32> = HashMap::new();
    for s in a {
        let it = st.entry(s).or_insert(0);
        *it += 1;
    }
    let v = vec!["AC", "WA", "TLE", "RE"];
    for k in v {
        let res = st.get(&k.to_string());
        match res {
            Some(x) => println!("{} x {}", k, x),
            None => println!("{} x {}", k, 0),
        }
    }
}

fn main() {
    solve();
}
