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

#[allow(non_snake_case)]
fn solve() {
    input! {
        h:usize,w:usize,d:usize,field:[Chars;h]
    }
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == 'H' {
                que.push_back((i, j));
            }
        }
    }
    let mut cnt = vec![vec![1e18 as usize; w]; h];
    for (r, c) in &que {
        cnt[*r][*c] = 0;
    }
    let dx = vec![0, 1, 0, !0];
    let dy = vec![1, 0, !0, 0];
    while let Some((r, c)) = que.pop_front() {
        for i in 0..4 {
            let nr = r.wrapping_add(dy[i]);
            let nc = c.wrapping_add(dx[i]);
            if nr >= h || nc >= w {
                continue;
            }
            if field[nr][nc] == '#' {
                continue;
            }
            if cnt[nr][nc] <= cnt[r][c] || cnt[r][c] + 1 > d || cnt[nr][nc] == 0 {
                continue;
            }
            cnt[nr][nc] = cnt[nr][nc].min(cnt[r][c] + 1);
            que.push_back((nr, nc));
            // println!(
            //     "{}\n----------------------------",
            //     que.iter()
            //         .map(|(x, y)| x.to_string() + " " + &y.to_string())
            //         .join("\n")
            // );
        }
    }
    let mut ans = 0;
    for i in 0..h {
        ans += cnt[i].iter().filter(|f| f != &&(1e18 as usize)).count();
        //    println!("{}", cnt[i].iter().join(" "));
    }
    println!("{}", ans);
}
fn main() {
    solve();
}
