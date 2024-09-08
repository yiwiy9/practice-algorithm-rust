use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    if l == 1 && r == 1 {
        println!("Invalid");
    } else if l == 1 {
        println!("Yes");
    } else if r == 1 {
        println!("No");
    } else {
        println!("Invalid");
    }
}
