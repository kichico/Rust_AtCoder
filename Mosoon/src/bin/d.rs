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
        n: i64,
        k: usize,
        D: [char;k],
    }
    let mut st: BTreeSet<char> = BTreeSet::new();
    for x in D {
        st.insert(x);
    }
    let mut ans: BTreeSet<i64> = BTreeSet::new();
    for i in n..100000 {
        let s = i.to_string();
        let cc: Vec<char> = s.as_str().chars().collect();
        let mut flg: bool = true;
        for x in cc {
            if st.contains(&x) == true {
                flg = false;
                break;
            }
        }
        if flg {
            ans.insert(i);
        }
    }
    println!("{}", ans.iter().next().unwrap());
}

fn main() {
    solve();
}
