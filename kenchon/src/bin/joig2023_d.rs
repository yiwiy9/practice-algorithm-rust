use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut column_head_cnt = vec![0; w];
    for row in &s {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                column_head_cnt[j] += 1;
            }
        }
    }

    let mut ans: usize = 0;
    for row in &s {
        let mut cur_column_head_cnt = column_head_cnt.clone();
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                cur_column_head_cnt[j] -= 1;
            } else {
                cur_column_head_cnt[j] += 1;
            }
        }
        let max_cur_column_head_cnt = *cur_column_head_cnt.iter().max().unwrap();
        let cur_head_cnt = cur_column_head_cnt.iter().sum::<usize>()
            + (h - max_cur_column_head_cnt)
            - max_cur_column_head_cnt;
        ans = ans.max(cur_head_cnt);
    }

    println!("{} {}", ans, h * w - ans);
}
