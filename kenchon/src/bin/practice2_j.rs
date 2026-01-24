use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

// fn main() {
//     input! {
//         n: usize,
//         q: usize,
//         a: [usize; n],
//     }

//     let mut max_segtree = Segtree::<Max<usize>>::from(a);

//     for _ in 0..q {
//         input! {
//             op:usize,
//         }
//         match op {
//             1 => {
//                 input! {
//                     x:Usize1,
//                     v:usize
//                 }
//                 max_segtree.set(x, v);
//             }
//             2 => {
//                 input! {
//                     l:Usize1,
//                     r:Usize1,
//                 }
//                 println!("{}", max_segtree.prod(l..=r));
//             }
//             3 => {
//                 input! {
//                     x:Usize1,
//                     v:usize
//                 }
//                 let idx = if v == 0 {
//                     // max_right(x, f) は "f(e()) == true" が必須
//                     // => v=0 のとき f(0) = (0 < 0) = false → 内部assertでpanic → RE
//                     // https://atcoder.github.io/ac-library/production/document_ja/segtree.html
//                     x
//                 } else {
//                     max_segtree.max_right(x, |&y| y < v)
//                 };
//                 println!("{}", idx + 1);
//             }
//             _ => unreachable!(),
//         }
//     }
// }

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut max_segtree = Segtree::<Max<i64>>::from(a);

    for _ in 0..q {
        input! {
            op:usize,
        }
        match op {
            1 => {
                input! {
                    x:Usize1,
                    v:i64
                }
                max_segtree.set(x, v);
            }
            2 => {
                input! {
                    l:Usize1,
                    r:Usize1,
                }
                println!("{}", max_segtree.prod(l..=r));
            }
            3 => {
                input! {
                    x:Usize1,
                    v:i64
                }
                // usize を使うと、e()=0 となってしまい使い勝手が悪い => i64 を使うと解決
                let idx = max_segtree.max_right(x, |&y| y < v);
                println!("{}", idx + 1);
            }
            _ => unreachable!(),
        }
    }
}
