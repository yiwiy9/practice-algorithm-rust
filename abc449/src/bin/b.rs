use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
        queries: [(usize,usize); q],
    }

    for &(op, size) in &queries {
        if op == 1 {
            println!("{}", size * w);
            h -= size;
        } else {
            println!("{}", size * h);
            w -= size;
        }
    }
}
