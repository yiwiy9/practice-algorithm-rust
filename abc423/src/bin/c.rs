use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [usize; n],
    }

    let mut l_one_continuous = 0;
    for &li in &l {
        if li == 1 {
            l_one_continuous += 1;
        } else {
            break;
        }
    }
    let mut r_one_continuous = 0;
    for &li in l.iter().rev() {
        if li == 1 {
            r_one_continuous += 1;
        } else {
            break;
        }
    }

    let mut left_ans = 0;
    if l_one_continuous + 1 < r {
        left_ans += l[l_one_continuous + 1..r]
            .iter()
            .filter(|&&x| x == 1)
            .count();
        left_ans += r - l_one_continuous;
    } else if l_one_continuous + 1 == r {
        left_ans += 1;
    }

    if n - r_one_continuous > r {
        left_ans += l[r..n - r_one_continuous]
            .iter()
            .filter(|&&x| x == 1)
            .count();
        left_ans += n - r_one_continuous - r;
    } else if n - r_one_continuous - 1 == r {
        left_ans += 1;
    }

    println!("{}", left_ans);
}
