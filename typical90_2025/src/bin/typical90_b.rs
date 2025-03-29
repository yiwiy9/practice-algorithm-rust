use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for bit in 0..(1 << n) {
        let mut res = vec![];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                res.push(')');
            } else {
                res.push('(');
            }
        }

        let mut ok = true;
        let mut cnt = 0;
        for i in 0..n {
            if res[i] == ')' {
                cnt -= 1;
            } else {
                cnt += 1;
            }
            if cnt < 0 {
                ok = false;
                break;
            }
        }
        if cnt == 0 && ok {
            ans.push(res);
        }
    }

    ans.sort();

    println!("{}", ans.iter().map(|x| x.iter().join("")).join("\n"));
}
