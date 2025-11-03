use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a <= c {
        if b <= d {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        println!("No");
    }
}
