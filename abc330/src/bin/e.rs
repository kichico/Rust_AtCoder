#[allow(unused_imports)]
use itertools::*;
use nalgebra::Reflection4;
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
use std::ops::RangeBounds;
#[allow(dead_code)]
#[allow(non_snake_case)]

fn to_char<T>(x: T) -> char
where
    T: PrimInt,
{
    return std::char::from_digit(x.to_u32().unwrap(), 10).unwrap();
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
    pub fn find(&self, target: &usize) -> Option<(usize, usize)> {
        let mut lower_bound = self
            .st
            .range((Unbounded, Excluded((target + 1, target + 1))));
        if let Some(value) = lower_bound.next_back() {
            if value.1 < *target {
                return None;
            } else {
                return Some(*value);
            }
        } else {
            return None;
        }
    }
    pub fn contains(&self, target: &usize) -> bool {
        if let Some(_value) = self.find(target) {
            return true;
        } else {
            return false;
        }
    }
    pub fn insert(&mut self, mut left: usize, mut right: usize) -> bool {
        if left >= right {
            return false;
        }
        let mut existing = self.st.range((Unbounded, Included((right, right))));
        let mut will_remove: Vec<(usize, usize)> = Vec::new();
        let mut range_len_diff = 0i64;
        while let Some((el, er)) = existing.next_back() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
                left = left.min(*el);
                right = right.max(*er);
            } else {
                break;
            }
        }
        let mut existing = self.st.range((Included((left, left)), Unbounded));
        while let Some((el, er)) = existing.next() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
                left = left.min(*el);
                right = right.max(*er);
            } else {
                break;
            }
        }
        if !will_remove.is_empty() {
            for p in will_remove {
                self.st.remove(&p);
                range_len_diff -= (p.1 - p.0) as i64;
            }
        }
        self.st.insert((left, right));
        range_len_diff += (right - left) as i64;
        self.range_length = (self.range_length as i64 + range_len_diff) as usize;
        return true;
    }
    pub fn remove(&mut self, left: usize, right: usize) {
        assert!(left <= right);
        let mut existing = self.st.range((Unbounded, Included((right, right))));
        let mut will_remove: Vec<(usize, usize)> = Vec::new();
        let mut range_len_diff = 0i64;
        while let Some((el, er)) = existing.next_back() {
            if *el <= right && left <= *er {
                will_remove.push((*el, *er));
            } else {
                break;
            }
        }
        if !will_remove.is_empty() {
            for p in will_remove {
                let (el, er) = p;
                self.st.remove(&p);
                range_len_diff -= (p.1 - p.0) as i64;
                if el <= left && right <= er {
                    self.insert(el, left);
                    self.insert(right, er);
                } else if left <= er && right <= er {
                    self.insert(el, left);
                } else if left <= el && right <= er {
                    self.insert(right, er);
                }
            }
        }
        self.range_length = (self.range_length as i64 + range_len_diff) as usize;
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,q:usize,mut a:[usize;n],query:[(Usize1,usize);q]
    }
    let mut rangeset = RangeSet::new();
    let mut mp = HashMap::new();
    for i in 0..n {
        *mp.entry(a[i]).or_insert(0) += 1;
        rangeset.insert(a[i], a[i] + 1);
    }
    for (i, x) in query {
        if let Some(prev) = mp.get(&a[i]) {
            if *prev == 1 {
                rangeset.remove(a[i], a[i] + 1);
                mp.remove(&a[i]);
            } else {
                let e = mp.get_mut(&a[i]).unwrap();
                *e -= 1;
            }
        } else {
            panic!();
        }
        a[i] = x;
        //println!("before:{:?}", rangeset.st);
        rangeset.insert(x, x + 1);
        *mp.entry(a[i]).or_insert(0) += 1;
        //println!("after:{:?}", rangeset.st);
        if let Some((_l, r)) = rangeset.find(&0) {
            println!("{}", r);
        } else {
            println!("{}", 0);
        }
    }
}
fn main() {
    solve();
}
