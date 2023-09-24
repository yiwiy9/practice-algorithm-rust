use proconio::input;

/**
 * https://atcoder.jp/contests/abc321/editorial/7261
 * 321-like Number の各桁に同じ数字が2回以上現れることはない
 * => 0~9に関してそれぞれ使う使わないをbit全探索 1022通り (2^10 - 何も使わない - 0しか使わない)
 * => 余裕で間に合う
 */
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
