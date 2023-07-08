use core::ops::Bound::*;
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
pub struct RangeSet {
    st: BTreeSet<(usize, usize)>,
    range_length: usize,
}
impl RangeSet {
    pub fn new() -> Self {
        let st: BTreeSet<(usize, usize)> = BTreeSet::new();
        let range_length = 0;
        return RangeSet { st, range_length };
    }
    pub fn len(&self) -> usize {
        return self.st.len();
    }
    pub fn range_len(&self) -> usize {
        return self.range_length;
    }
    pub fn find(&self, kth: &usize) -> Option<(usize, usize)> {
        let mut lower_bound = self.st.range((Unbounded, Excluded((kth + 1, kth + 1))));
        let value = lower_bound.next_back();
        match value {
            Some(ret) => {
                if ret.1 < *kth {
                    return None;
                } else {
                    return value.copied();
                }
            }
            None => return None,
        }
    }
    pub fn contains(&self, kth: &usize) -> bool {
        let value = self.find(kth);
        match value {
            Some(_ret) => return true,
            None => return false,
        }
    }
    pub fn insert(&mut self, mut left: usize, mut right: usize) {
        assert!(left <= right);

        self.st.insert((left, right));
    }
    pub fn remove(&mut self, left: usize, right: usize) {
        assert!(left <= right);
        self.range_length -= right - left + 1;
        match self.find(&left) {
            Some((u, v)) => {
                self.st.remove(&(u, v));
                if left != u {
                    self.insert(u, left - 1);
                }
                if right != v {
                    self.insert(right + 1, v);
                }
            }
            None => panic!("out of range"),
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,a:[usize;n],q:usize,query:[(usize,usize);q]
    }
    let mut sleep_wake: RangeSet = RangeSet::new();
    let mut cumsum = vec![0; n];
    for i in 1..n {
        cumsum[i] = cumsum[i - 1];
        if i % 2 == 0 && i != 0 {
            cumsum[i] += a[i] - a[i - 1];
        }
        if i < n - 1 && i % 2 == 1 {
            sleep_wake.insert(a[i], a[i + 1]);
        }
    }
    let mut ans = vec![0; q];
    for (i, (from, to)) in enumerate(query) {
        let left = a.lower_bound(&from);
        let right = a.lower_bound(&to);
        ans[i] += cumsum[right] - cumsum[left];
        println!("l:{} r:{}", left, right);
        println!("{} {}", cumsum[left], cumsum[right]);
        if let Some(p) = sleep_wake.find(&from) {
            if p.0 <= from && from < p.1 {
                ans[i] -= from - p.0;
            }
        }
        if let Some(p) = sleep_wake.find(&to) {
            ans[i] += p.1 - to;
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
