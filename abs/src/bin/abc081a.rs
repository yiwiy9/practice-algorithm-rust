use proconio::{input, marker::Chars};

// fn main() {
//     input! {
//         s: String,
//     }

//     let mut count = 0;
//     for c in s.chars() {
//         match c {
//             '1' => count += 1,
//             '0' => continue,
//             _ => panic!(),
//         }
//     }
//     println!("{}", count);
// }

fn main() {
    input! {
        s: Chars,
    }

    let mut count = 0;
    for c in s {
        if c == '1' {
            count += 1;
        }
        // match c {
        //     '1' => count += 1,
        //     '0' => continue,
        //     _ => panic!(),
        // }
    }
    println!("{}", count);
}
