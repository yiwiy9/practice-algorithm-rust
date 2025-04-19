use proconio::input;

fn main() {
    input! {
        mut v: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    loop {
        if v < a {
            println!("F");
            break;
        }
        v -= a;
        if v < b {
            println!("M");
            break;
        }
        v -= b;
        if v < c {
            println!("T");
            break;
        }
        v -= c;
    }
}
