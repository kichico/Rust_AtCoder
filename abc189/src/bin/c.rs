#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
#[allow(unused_imports)]
use petgraph::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        n: usize,a:[i64;n]
    }
    let mut right = 1;
    let mut ans = 0;
    let mut st: BTreeMap<i64, i64> = BTreeMap::new();
    for left in 0..n {
        let mut current = a[left];
        let it = st.entry(a[left]).or_insert(0);
        *it += 1;
        let mini = *st.iter().next().unwrap().0;
        while right < n && min(a[right], mini) * (right - left + 1) as i64 >= current {
            let it = st.entry(a[right]).or_insert(0);
            *it += 1;
            let mini = *st.iter().next().unwrap().0;
            current = (right - left + 1) as i64 * mini;
            right += 1;
        }
        ans = ans.max(current);
        dbg!(mini);
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
