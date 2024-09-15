use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, char); m],
    }

    let mut has_taro = vec![false; n];
    for (a, b) in ab {
        if b == 'M' && !has_taro[a] {
            println!("Yes");
            has_taro[a] = true;
        } else {
            println!("No");
        }
    }
}
