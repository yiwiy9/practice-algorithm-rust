use num::integer::gcd;
use proconio::input;

const UPPER: u128 = 1000000000000000000;

fn main() {
    input! {
        a: u128,
        b: u128,
    }

    let s: u128 = a * b / gcd(a, b);

    if s <= UPPER {
        println!("{}", s)
    } else {
        println!("Large")
    }
}

// const UPPER: u64 = 1000000000000000000;

// fn main() {
//     input! {
//         a: u64,
//         b: u64,
//     }

//     let d = gcd(a, b);
//     let a_d = a / d;

//     if a_d <= UPPER / b {
//         println!("{}", a_d * b)
//     } else {
//         println!("Large")
//     }
// }
