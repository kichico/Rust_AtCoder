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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,
        pos: [(i64,i64);n],
        ss: String,
    }
    let s: Vec<char> = ss.chars().collect();
    let mut dic: HashMap<(i64, char), i64> = HashMap::new();
    let mut list = HashSet::new();
    for i in 0..n {
        dic.entry((pos[i].1, s[i]))
            .and_modify(|e| {
                *e = if s[i] == 'L' {
                    max(*e, pos[i].0)
                } else {
                    min(*e, pos[i].0)
                }
            })
            .or_insert(pos[i].0);
        list.insert(pos[i].1);
    }
    for p in list {
        let left = dic.get(&(p, 'R'));
        let right = dic.get(&(p, 'L'));
        if left.is_none() || right.is_none() {
            continue;
        }
        if left < right {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    solve();
}
