use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut ans = INF;

    for i in 0..h {
        let mut dot_cnt = 0;
        let mut ok_cnt = 0;
        let mut ng_cnt = 0;
        for j in 0..w {
            match s[i][j] {
                '.' => {
                    dot_cnt += 1;
                }
                'o' => {
                    ok_cnt += 1;
                }
                'x' => {
                    ng_cnt += 1;
                }
                _ => unreachable!(),
            }

            if dot_cnt + ok_cnt + ng_cnt == k {
                if ng_cnt == 0 {
                    ans = ans.min(dot_cnt);
                }
                match s[i][j + 1 - k] {
                    '.' => {
                        dot_cnt -= 1;
                    }
                    'o' => {
                        ok_cnt -= 1;
                    }
                    'x' => {
                        ng_cnt -= 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    for j in 0..w {
        let mut dot_cnt = 0;
        let mut ok_cnt = 0;
        let mut ng_cnt = 0;
        for i in 0..h {
            match s[i][j] {
                '.' => {
                    dot_cnt += 1;
                }
                'o' => {
                    ok_cnt += 1;
                }
                'x' => {
                    ng_cnt += 1;
                }
                _ => unreachable!(),
            }

            if dot_cnt + ok_cnt + ng_cnt == k {
                if ng_cnt == 0 {
                    ans = ans.min(dot_cnt);
                }
                match s[i + 1 - k][j] {
                    '.' => {
                        dot_cnt -= 1;
                    }
                    'o' => {
                        ok_cnt -= 1;
                    }
                    'x' => {
                        ng_cnt -= 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    println!("{:?}", if ans == INF { -1 } else { ans as i64 });
}
