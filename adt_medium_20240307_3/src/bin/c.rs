use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut easy = true;
    loop {
        let a_num = a % 10;
        let b_num = b % 10;

        if a_num + b_num >= 10 {
            easy = false;
            break;
        }

        a /= 10;
        b /= 10;

        if a == 0 || b == 0 {
            break;
        }
    }

    println!("{}", if easy { "Easy" } else { "Hard" });
}
