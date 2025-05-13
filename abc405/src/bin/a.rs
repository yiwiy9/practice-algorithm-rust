use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
    }

    if x == 1 {
        if 1600 <= r && r < 3000 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if 1200 <= r && r < 2400 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
