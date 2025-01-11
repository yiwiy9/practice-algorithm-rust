use proconio::input;

/// https://atcoder.jp/contests/adt_all_20240314_1/tasks/abc321_c
/// https://atcoder.jp/contests/adt_all_20240314_1/editorial/7261
/// 重要な観察として、 321-like Number の各桁に同じ数字が 2 回以上現れることはありません。
/// その数字を含むか含まないかの 2 通りがあるため、全体で 2^10 = 1024 通りです。
/// bit全探索で十分間に合う。
/// あとで、ソートすれば良い。
fn main() {
    input! {
        k: usize,
    }
    let mut like_numbers = vec![];

    // bit 0,1 は除外ケース
    for bit in 2..(1 << 10) {
        let mut num = 0usize;
        for i in (0..=9).rev() {
            if bit & (1 << i) != 0 {
                num *= 10;
                num += i;
            }
        }
        like_numbers.push(num);
    }

    like_numbers.sort();
    println!("{}", like_numbers[k - 1]);
}
