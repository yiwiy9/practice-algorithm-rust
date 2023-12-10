use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut eating = 0;
    let mut conp = 0;

    for &c in &s {
        match c {
            '0' => {
                let mut cur = conp;
                if eating > m {
                    cur += eating - m;
                }
                ans = ans.max(cur);

                eating = 0;
                conp = 0;
            }
            '1' => {
                eating += 1;
            }
            '2' => {
                conp += 1;
            }
            _ => unreachable!(),
        }
    }
    let mut cur = conp;
    if eating > m {
        cur += eating - m;
    }
    ans = ans.max(cur);

    println!("{}", ans);
}
