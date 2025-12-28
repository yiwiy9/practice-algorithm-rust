use proconio::input;

fn main() {
    input! {
        d: usize,
        f: usize,
    }

    let mut cur = f;
    loop {
        cur += 7;
        if cur > d {
            println!("{}", cur % d);
            return;
        }
    }
}
