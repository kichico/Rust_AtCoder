#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
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
#[allow(unused_imports)]
use std::mem::swap;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,mut a:[i64;n]
    }
    a.sort();
    let dist = a[1] - a[0];
    let mut st: BTreeSet<i64> = BTreeSet::new();
    st.insert(dist);
    for i in 2..n {
        st.insert(a[i] - a[i - 1]);
    }
    let mut iter = st.iter();
    let mut gcd_value = iter.next().unwrap().clone();
    for v in iter {
        gcd_value = gcd(gcd_value, *v);
    }
    dbg!(&gcd_value);
    if gcd_value != 1 {
        println!("1");
    } else {
        println!("2");
    }
}
fn main() {
    solve();
}
