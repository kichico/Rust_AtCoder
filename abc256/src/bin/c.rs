#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use num_integer::Roots;
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
#[allow(non_snake_case)]
fn solve() {
    input! {
        h: [i64;3], w:[i64;3]
    }
    let mut field = vec![vec![0; 3]; 3];
    let n = 3;
    let mut ans = 0;
    for h0w0 in 1..30 {
        field[0][0] = h0w0;
        for h1w0 in 1..30 {
            field[1][0] = h1w0;
            for h0w1 in 1..30 {
                field[0][1] = h0w1;
                for h1w1 in 1..30 {
                    field[1][1] = h1w1;
                    for r in 0..2 {
                        field[r][2] = h[r] - field[r][1] - field[r][0];
                    }
                    for c in 0..3 {
                        field[2][c] = w[c] - field[1][c] - field[0][c];
                    }
                    if field[2][2] + field[2][1] + field[2][0] != h[2] {
                        continue;
                    }
                    let mut flg = true;
                    for i in 0..n {
                        for j in 0..n {
                            print!("{} ", field[i][j]);
                            if field[i][j] <= 0 {
                                flg = false;
                            }
                        }
                        println!();
                    }
                    if flg {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    solve();
}
