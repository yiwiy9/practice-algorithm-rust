use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if b < c {
        if b <= a && a <= c {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        if b <= a || a <= c {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
