use memoise::memoise_map;
use proconio::input;
// use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }

    // println!("{}", rec(n, &mut HashMap::new()));
    println!("{}", rec_memo(n));
}

// fn rec(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
//     if let Some(&res) = memo.get(&n) {
//         return res;
//     }
//     if n == 0 {
//         return 1;
//     }

//     let res = rec(n / 2, memo) + rec(n / 3, memo);
//     memo.insert(n, res);
//     res
// }

#[memoise_map(n)]
fn rec_memo(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        rec_memo(n / 2) + rec_memo(n / 3)
    }
}
