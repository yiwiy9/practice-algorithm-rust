use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let ab = i;
        let cd = n - i;

        let mut ab_cnt = 0;
        for j in 1..=((ab as f64).sqrt() as usize) {
            if ab % j == 0 {
                ab_cnt += 1;
                if j * j != ab {
                    ab_cnt += 1;
                }
            }
        }

        let mut cd_cnt = 0;
        for j in 1..=((cd as f64).sqrt() as usize) {
            if cd % j == 0 {
                cd_cnt += 1;
                if j * j != cd {
                    cd_cnt += 1;
                }
            }
        }

        ans += ab_cnt * cd_cnt;
    }

    println!("{}", ans);
}
