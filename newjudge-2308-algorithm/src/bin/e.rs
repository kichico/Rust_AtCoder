#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::hash::Hash;
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
use ac_library::dsu::Dsu;
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,edge:[(Usize1,Usize1);m],k:usize,not_good:[(Usize1,Usize1);k],q:usize,query:[(Usize1,Usize1);q]
    }
    let mut uf = Dsu::new(n);
    for (u, v) in edge {
        uf.merge(u, v);
    }
    let mut ans = vec!["0"; q];
    let mut check: HashSet<(usize, usize)> = HashSet::new();
    for (u, v) in &not_good {
        if !uf.same(*u, *v) {
            let up = uf.leader(*u);
            let vp = uf.leader(*v);
            check.insert((up.min(vp), up.max(vp)));
        }
    }
    for (i, (u, v)) in enumerate(query) {
        let u = uf.leader(u);
        let v = uf.leader(v);
        ans[i] = if check.contains(&(u.min(v), u.max(v))) {
            "No"
        } else {
            "Yes"
        };
    }
    for s in ans {
        println!("{}", s);
    }
}
fn main() {
    solve();
}
