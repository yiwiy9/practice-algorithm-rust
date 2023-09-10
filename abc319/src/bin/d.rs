use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    }

    let mut left = *l.iter().max().unwrap() - 1;
    let mut right = 1_usize << 60;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut column_cnt = l[0];
        let mut row_cnt = 1_usize;
        for &l_i in l.iter().skip(1) {
            if column_cnt + 1 + l_i <= mid {
                column_cnt += 1 + l_i;
            } else {
                column_cnt = l_i;
                row_cnt += 1;
            }
        }

        if row_cnt > m {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}
