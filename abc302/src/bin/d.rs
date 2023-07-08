#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::*;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
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
use superslice::Ext;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,d:i64,mut t:[i64;n],mut a:[i64;m]
    }
    let mut ans = -1;
    t.sort();
    a.sort();
    for val in t {
        let rest = val + d;
        let i = a.lower_bound(&rest);
        if i < m && abs(val - a[i]) <= d {
            ans = ans.max(a[i] + val);
        }
        if i + 1 < m && abs(val - a[i + 1]) <= d {
            ans = ans.max(a[i + 1] + val);
        }
        if let Some(x) = i.checked_sub(1) {
            if abs(val - a[x]) <= d {
                ans = ans.max(a[x] + val);
            }
        }
        let rest = val - d;
        let i = a.lower_bound(&rest);
        if i < m && abs(val - a[i]) <= d {
            ans = ans.max(a[i] + val);
        }
        if i + 1 < m && abs(val - a[i + 1]) <= d {
            ans = ans.max(a[i + 1] + val);
        }
        if let Some(x) = i.checked_sub(1) {
            if abs(val - a[x]) <= d {
                ans = ans.max(a[x] + val);
            }
        }
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
