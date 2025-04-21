#[allow(unused_imports)]
use itertools::*;
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
use std::collections::binary_heap;
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
#[allow(non_snake_case)]
fn solve() {
    input! {
        n:usize,m:usize,score:[usize;m],tokuten:[Chars;n]
    }
    let mut ans = vec![0; n];
    let mut rest: HashSet<usize> = HashSet::from_iter((0..n).into_iter());
    let mut current: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut score_cnt = vec![0; 2500 * m + 10000];
    for i in 0..n {
        let it = tokuten[i]
            .iter()
            .enumerate()
            .filter(|x| x.1 == &'o')
            .map(|x| x.0);
        let ith_score = it.map(|x| score[x]).sum::<usize>();
        current.insert((ith_score + i + 1, i));
        score_cnt[ith_score + i + 1] += 1;
    }
    'a: for (mut ith_score, i) in &current {
        let it = tokuten[*i]
            .iter()
            .enumerate()
            .filter(|x| x.1 == &'o')
            .map(|x| x.0);
        let mut not_solved: HashSet<usize> = HashSet::from_iter((0..m).into_iter());
        for c in it {
            not_solved.remove(&c);
        }
        let mut avail: BinaryHeap<usize> = BinaryHeap::new();
        for ns in not_solved {
            avail.push(score[ns]);
        }
        let mut cnt = 0;

        if i == &current.iter().last().unwrap().1 && score_cnt[ith_score] == 1 {
            ans[*i] = 0;
            continue;
        }
        while let Some(x) = avail.pop() {
            cnt += 1;
            ith_score += x;
            if ith_score > current.iter().last().unwrap().0 {
                ans[*i] = cnt;
                continue 'a;
            }
        }
    }
    println!("{}", ans.iter().join("\n"));
}
fn main() {
    solve();
}
