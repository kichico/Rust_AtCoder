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
    pub fn remove_all(&mut self, x: T) -> Option<T> {
        if let Some(_x) = self.check.get(&x) {
            self.counter.remove(&x);
            self.check.remove(&x);
            return Some(x);
        } else {
            None
        }
    }
    pub fn clear(&mut self) -> bool {
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
    pub fn show(&self)
    where
        T: std::fmt::Display,
    {
        if self.check.is_empty() {
            println!("None");
        } else {
            let iter = self.check.iter().clone();
            for v in iter {
                let num_iteration = self.counter.get(v).unwrap();
                for _n in 0..*num_iteration {
                    print!("{} ", v);
                }
            }
            println!();
        }
    }
}

#[allow(non_snake_case)]
fn prime_factorize(mut x: i64) -> Vec<(i64, i64)> {
    let mut factor: Vec<(i64, i64)> = Vec::new();
    let sq = x.sqrt() + 1;
    let mut isSeeked = vec![true; sq as usize + 1];
    for i in 2..=sq {
        if !isSeeked[i as usize] {
            continue;
        }
        let mut exp_num = 0;
        while x % i == 0 {
            exp_num += 1;
            x /= i;
        }
        for n in (i * 2..=sq).step_by(i as usize) {
            isSeeked[n as usize] = false;
        }
        if exp_num != 0 {
            factor.push((i, exp_num));
        }
    }
    if x > 1 {
        factor.push((x, 1));
    }
    return factor;
}
#[allow(non_snake_case)]
fn solve() {
    input! {
        mut k:i64
    }
    let factors = prime_factorize(k);
    let mut ans = 0;
    for (p, mut a) in factors {
        let mut n = 0;
        while a > 0 {
            n += p;
            let mut x = n.clone();
            while x % p == 0 {
                x /= p;
                a -= 1;
            }
        }
        ans = ans.max(n);
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
