#[allow(unused_imports)]
use itertools::*;
use itertools_num::ItertoolsNum;
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
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(unused_imports)]
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[derive(Debug, Clone)]
pub struct MultiSet<T>
where
    T: Copy + Clone + Hash + Eq + Ord,
{
    check: BTreeSet<T>,
    counter: HashMap<T, i64>,
}
impl<T> MultiSet<T>
where
    T: Copy + Clone + Hash + Eq + Ord,
{
    pub fn new() -> Self {
        let check: BTreeSet<T> = BTreeSet::new();
        let counter: HashMap<T, i64> = HashMap::new();
        return MultiSet { check, counter };
    }
    pub fn max(&self) -> Option<&T> {
        if let Some(max_value) = self.check.iter().next_back() {
            return Some(max_value);
        } else {
            return None;
        }
    }
    pub fn min(&self) -> Option<&T> {
        if let Some(min_value) = self.check.iter().next() {
            return Some(min_value);
        } else {
            return None;
        }
    }
    pub fn insert(&mut self, x: T) -> Option<T>
    where
        T: Copy + Clone + Ord + Hash,
    {
        if let Some(_i) = self.check.get(&x) {
            *self.counter.entry(x).or_insert(0) += 1;
        } else {
            self.check.insert(x);
            *self.counter.entry(x).or_insert(0) += 1;
        }
        return Some(x);
    }
    pub fn contains(&self, x: T) -> bool
    where
        T: Copy + Clone + Ord + Hash,
    {
        if let Some(_x) = self.check.get(&x) {
            return true;
        } else {
            return false;
        }
    }
    pub fn remove(&mut self, x: T) -> Option<T>
    where
        T: Copy + Clone + Ord + Hash,
    {
        if let Some(_x) = self.check.get(&x) {
            if *self.counter.get(&x).unwrap() == 1 {
                self.counter.remove(&x);
                self.check.remove(&x);
                return Some(x);
            } else {
                let v = self.counter.get_mut(&x);
                match v {
                    Some(v) => *v -= 1,
                    None => (),
                }
                return Some(x);
            }
        } else {
            None
        }
    }
    pub fn remove_all(&mut self, x: T) -> Option<T>
    where
        T: Copy + Clone + Ord + Hash,
    {
        if let Some(_x) = self.check.get(&x) {
            self.counter.remove(&x);
            self.check.remove(&x);
            return Some(x);
        } else {
            None
        }
    }
    pub fn clear(&mut self) -> bool
    where
        T: Copy + Clone + Ord + Hash,
    {
        if !self.check.is_empty() {
            self.check.clear();
            self.counter.clear();
            return true;
        } else {
            return false;
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.check.is_empty() {
            true
        } else {
            false
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,mut b:[i64;n],mut w:[i64;m]
    }
    b.sort();
    w.sort();
    b.reverse();
    w.reverse();
    let b_sum = vec![vec![0i64], b.iter().cumsum::<i64>().collect_vec()].concat();
    let w_sum = vec![vec![0i64], w.iter().cumsum::<i64>().collect_vec()].concat();
    let mut max_w = w_sum.clone();
    for i in 0..=m - 1 {
        max_w[i + 1] = max_w[i + 1].max(max_w[i]);
    }
    let mut ans = 0;
    for i in 0..=n {
        ans = ans.max(b_sum[i] + max_w[i.min(m)]);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
