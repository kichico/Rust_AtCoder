#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
#[allow(non_snake_case)]
#[fastout]

fn solve() {
    input! {
        n: usize,
        m: usize,
        a: [i64;n],
        b: [i64;n],
        edge: [(Usize1,Usize1);m],
    }
    let mut uf: UnionFind<usize> = UnionFind::new(n);
    for (x, y) in edge {
        uf.union(x, y);
    }
    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
    for x in 0..n {
        map.entry(uf.find(x) as i64).or_default().push(x as i64);
    }
    for gp in map.values() {
        let (mut asum, mut bsum): (i64, i64) = (0, 0);
        for i in 0..gp.len() {
            asum += a[gp[i] as usize];
            bsum += b[gp[i] as usize];
        }
        if asum != bsum {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn main() {
    solve();
}
