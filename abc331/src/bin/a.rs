use proconio::input;

fn main() {
    input! {
        mm: usize,
        dd: usize,
        ymd: (usize,usize,usize),
    }

    let (mut y, mut m, mut d) = ymd;

    d += 1;
    if d > dd {
        m += 1;
        d = 1;
    }
    if m > mm {
        y += 1;
        m = 1;
    }

    println!("{} {} {}", y, m, d);
}
