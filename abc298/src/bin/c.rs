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
    cnt: i64,
}
impl<T> MultiSet<T>
where
    T: Copy + Clone + Hash + Eq + Ord,
{
    pub fn new() -> Self {
        let check: BTreeSet<T> = BTreeSet::new();
        let counter: HashMap<T, i64> = HashMap::new();
        let cnt = 0;
        return MultiSet {
            check,
            counter,
            cnt,
        };
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
    pub fn show_all(&self) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();
        for elem in &self.check {
            let num = *self.counter.get(elem).unwrap();
            for i in 0..num {
                ret.push(*elem);
            }
        }
        return ret;
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
        n:usize,q:usize,
    }
    let mut hako: Vec<MultiSet<usize>> = vec![MultiSet::new(); 1e6 as usize];
    let mut kazu: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 1e6 as usize];
    for que in 0..q {
        input! {x:i64}
        if x == 1 {
            input! { i:usize,j:usize }
            hako[j].insert(i);
            kazu[i].insert(j);
        } else if x == 2 {
            input! { i:usize}
            println!("{}", hako[i].show_all().iter().join(" "));
        } else {
            input! { i:usize}
            println!("{}", kazu[i].iter().join(" "));
        }
    }
}
fn main() {
    solve();
}
