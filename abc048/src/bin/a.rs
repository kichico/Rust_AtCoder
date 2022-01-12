use proconio::input;

fn solve() {
    input! {
        _a: String,
        b: String,
        _c: String,
    }
    println!("A{}C", b.chars().nth(0).unwrap());
}

fn main() {
    solve();
}
