use ac_library::{Additive, Max, Segtree};
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc368/tasks/abc368_g
/// https://atcoder.jp/contests/abc368/editorial/10764
///
/// 重要な考察は、ここから v を v×B[i] で置き換える操作を選ぶ回数は、
/// 雑に見積もっても log2 M (<60) 回しかないことです。
/// なぜなら、2^60 > 10^18 なので 60 回以上この操作をできる入力が与えられないからです。
///
/// => Bのほとんどが1であることが推測できる
/// => モノイドではないので全てをセグ木に乗せることはできないが、
///    制約上、Bを選ぶ可能性のある点（>=2）で全探索は可能である
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
    }

    let mut a_sum_segtree = Segtree::<Additive<usize>>::from(a);
    let mut b_max_segtree = Segtree::<Max<usize>>::from(b);

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    i: Usize1,
                    x: usize,
                }
                a_sum_segtree.set(i, x);
            }
            2 => {
                input! {
                    i: Usize1,
                    x: usize,
                }
                b_max_segtree.set(i, x);
            }
            3 => {
                input! {
                    mut l: Usize1,
                    r: Usize1,
                }

                let mut v = a_sum_segtree.get(l);
                while l < r {
                    // min(r, l + 1 以降で B[i] >= 2 であるような最小の i)
                    let next_l = b_max_segtree.max_right(l + 1, |&x| x < 2).min(r);

                    // B[i] = 1 の区間は A[i] を足す
                    v += a_sum_segtree.prod(l + 1..next_l);

                    v = (v + a_sum_segtree.get(next_l)).max(v * b_max_segtree.get(next_l));
                    l = next_l;
                }

                println!("{}", v);
            }
            _ => unreachable!(),
        }
    }
}
