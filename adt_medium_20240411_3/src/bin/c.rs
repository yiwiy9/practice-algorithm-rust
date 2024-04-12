use proconio::input;

fn main() {
    input! {
        b: usize,
    }

    for a in 1_usize..16 {
        if a.pow(a as u32) == b {
            println!("{}", a);
            return;
        }
    }

    println!("-1");
}
