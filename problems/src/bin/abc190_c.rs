use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
        k: usize,
        cd: [(Usize1,Usize1); k],
    }

    let mut ans = 0;
    for bit in 0..(1 << k) {
        let mut dishes = vec![false; n];
        for i in 0..k {
            if bit & (1 << i) == 0 {
                dishes[cd[i].0] = true;
            } else {
                dishes[cd[i].1] = true;
            }
        }

        let mut cnt = 0;
        for &(a, b) in &ab {
            if dishes[a] && dishes[b] {
                cnt += 1;
            }
        }
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
