use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    if l <= 295 {
        if r <= 295 {
            println!("0");
        } else if r <= 416 {
            println!("1");
        } else {
            println!("2");
        }
    } else if l <= 416 {
        if r <= 416 {
            println!("0");
        } else {
            println!("1");
        }
    } else {
        println!("0");
    }
}
