#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn dfs(now:usize)


fn solve() {
    input! {
        n: usize,m:usize,edge:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut trueEdge: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in edge {
        let res = trueEdge.get(&(x, y));
        match res {
            None => {
                g[x].push(y);
                trueEdge.insert((x, y));
            }
            Some(n) => (),
        }
    }

}

fn main() {
    solve();
}
