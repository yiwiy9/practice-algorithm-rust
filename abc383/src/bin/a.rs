use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(i32,i32); n],
    }

    let mut water = 0;
    let mut before_t = 0;
    for &(t, v) in &tv {
        water = (water - (t - before_t)).max(0);
        water += v;
        before_t = t;
    }

    println!("{}", water);
}
