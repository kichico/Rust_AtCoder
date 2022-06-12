#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn solve() {
    input! {
        mut hour: i32,
        minute: i32
    }
    hour %= 12;
    let harg = (hour as f32 + minute as f32 / 60f32) * 30f32;
    let marg = minute as f32 * 6f32;
    dbg!(&harg, &marg);
    let sm = harg.min(marg);
    let lr = harg.max(marg);
    println!("{:.5}", lr - sm);
}

fn main() {
    solve();
}
