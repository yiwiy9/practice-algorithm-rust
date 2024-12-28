use proconio::input;

/// https://atcoder.jp/contests/adt_all_20241225_1/tasks/abc361_b
/// https://atcoder.jp/contests/adt_all_20241225_1/editorial/10353
/// x,y,z の 3 つの座標それぞれで 1 次元の問題を解き、
/// 全てで共通部分の長さが正なら、直方体の共通部分の体積は正、そうでなければ 0 です。
fn main() {
    input! {
        abc: (usize,usize,usize),
        def: (usize,usize,usize),
        ghi: (usize,usize,usize),
        jkl: (usize,usize,usize),
    }

    let has_length = |l1: usize, r1: usize, l2: usize, r2: usize| !(r1 <= l2 || r2 <= l1);

    println!(
        "{}",
        if has_length(abc.0, def.0, ghi.0, jkl.0)
            && has_length(abc.1, def.1, ghi.1, jkl.1)
            && has_length(abc.2, def.2, ghi.2, jkl.2)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
