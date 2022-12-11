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
use rand::Rng;
#[allow(non_snake_case)]
#[allow(unused)]
fn solve() {
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    input!(
        from &mut stdin,
        M:usize,eps:String
    );
    let N = 100;
    let mut all_graph: Vec<Vec<HashSet<i64>>> = vec![vec![HashSet::new(); N]; M];
    let mut ans: Vec<Vec<char>> = vec![Vec::new(); M];
    let mut current = 100;
    let mut cluster = 1;
    for i in 0..M {
        let edge_num = current / cluster;
        for nn in 0..cluster {}
    }
    println!("{}", N);
    for i in 0..M {
        println!("{}", ans[i].iter().join(""));
    }
    for _k in 0..100 {
        input! {
            from &mut stdin,
            g:Chars
        }
        if eps != "0.00" {
            println!("1");
        } else {
            let mut cnt = 0;
            for i in 0..g.len() {
                if g[i] == '1' {
                    cnt += 1;
                }
            }
            println!("{}", cnt);
        }
    }
}
fn main() {
    solve();
}
