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
use superslice::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
fn to_char(x: i64) -> char {
    return std::char::from_digit(x as u32, 10).unwrap();
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        h:usize,w:usize,rs:usize,cs:usize,n:usize,mut wall:[(usize,usize);n],q:usize
    }
    let mut row: HashMap<usize, VecDeque<usize>> = HashMap::new();
    let mut col: HashMap<usize, VecDeque<usize>> = HashMap::new();
    wall.sort();
    for (r, c) in &wall {
        row.entry(*r).or_insert(VecDeque::new()).push_back(*c);
    }
    wall.sort_by(|a, b| a.1.cmp(&b.1));
    for (r, c) in &wall {
        col.entry(*c).or_insert(VecDeque::new()).push_back(*r);
    }
    for (_, mut v) in row {
        v.push_front(0);
        v.push_back(w + 1);
    }
    for (_, mut v) in col {
        v.push_front(0);
        v.push_back(h + 1);
    }
    let mut r = rs as i64;
    let mut c = cs as i64;
    let mut dir: HashMap<char, usize> = HashMap::new();
    dir.insert('L', 0);
    dir.insert('R', 1);
    dir.insert('U', 2);
    dir.insert('D', 3);
    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    let mut ans: Vec<(i64, i64)> = Vec::new();
    for _ in 0..q {
        input! { d:char,dist:i64 }
        let k = *dir.get(&d).unwrap();
        let mut nr = r;
        let mut nc = c;
        if d == 'L' {
        } else if d == 'R' {
        } else if d == 'U' {
        } else {
        }
        r = nr;
        c = nc;
        //println!("{} {}", r + 1, c + 1);
        ans.push((r + 1, c + 1));
    }
    for (r, c) in ans {
        println!("{} {}", r, c);
    }
}

fn main() {
    solve();
}
