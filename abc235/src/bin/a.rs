#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
    read,
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: String,
    }
    let mut V: VecDeque<char> = n.as_str().chars().collect();
    let mut ans: i64 = 0;
    for _i in 0..3 {
        let mut c = String::new();
        for x in 0..3 {
            c.push(V[x]);
        }
        let cc: i64 = c.parse().unwrap();
        ans += cc;
        let nn = V.iter().next().unwrap();
        V.push_back(*nn);
        V.pop_front();
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
