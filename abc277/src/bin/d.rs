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
use std::ops::Bound::{Excluded, Included, Unbounded};
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
        let mut small = self.st.range((Unbounded, Excluded((left + 1, left + 1))));
        while let Some((u, v)) = small.next_back() {
            if left <= (*v) + 1 {
                left = min(*u, left);
            }
        }
        let mut large = self.st.range((Included((left, right + 1)), Unbounded));
        while let Some((u, v)) = large.next() {
            if *u == 0 || right >= (*u) - 1 {
                right = max(right, *v);
            }
        }
        let will_remove = self
            .st
            .range((Included((left, left)), Included((right, right))));
        let mut remover: Vec<(usize, usize)> = Vec::new();
        for rem in will_remove {
            remover.push(*rem);
            //    self.range_length -= rem.1 - rem.0 + 1;
        }

        for rem in remover {
            self.st.remove(&rem);
            //  self.range_length += rem.1 - rem.0 + 1;
        }
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
        n:usize,MOD:i64,mut a:[i64;n]
    }
    let mut org = a.clone();
    for i in 0..n {
        a[i] %= MOD;
    }
    a.sort();
    let mut v = a.clone();
    for i in 0..n {
        v.push(a[i]);
    }
    let mut longest = 0;
    let mut ans = (0, 0);
    let mut left = 0;
    let mut right = 1;
    if n == 1 {
        println!("0");
        return;
    }
    let mut rg = RangeSet::new();
    loop {
        if right >= n {
            break;
        }
        let mut last_value = v[left].clone();
        right = right.max(left + 1);
        while right < 2 * n && ((last_value + 1) % MOD == v[right] || last_value == v[right]) {
            //println!("last value :{} {}", last_value, v[right + 1]);
            last_value = v[right.clone()];
            right += 1;
            //println!("last value :{} {}", last_value, v[right]);
        }
        rg.insert(left, right - 1);

        //println!("left : {} right : {}", ans.0, ans.1);
        left += 1;
    }
    org.sort();
    for (left, right) in rg.st {
        println!("{} {}", left, right);
    }
}
fn main() {
    solve();
}
