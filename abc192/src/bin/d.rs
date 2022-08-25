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
fn Binary_to_dec(mut s: Vec<char>, base: i128) -> i128 {
    let mut ret: i128 = 0;
    s.reverse();
    let mut mp = base.pow(s.len() as u32 - 1);
    for i in 0..s.len() {
        let x = s[i] as i128;
        ret+=x*mp;
        mp/=base;
    }
    return ret;
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n: usize,
    }
}

fn main() {
    solve();
}
