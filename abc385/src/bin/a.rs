use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if (a == b && b == c) || (a + b == c) || (b + c == a) || (c + a == b) {
        println!("Yes");
        return;
    }

    println!("No");
}
