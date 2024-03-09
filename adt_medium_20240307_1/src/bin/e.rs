use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let mut problem_vec = vec![vec![]; 26];
    for (i, &a_i) in a.iter().enumerate() {
        problem_vec[a_i / 100].push(i);
    }

    let mut cur_scores = vec![0; n];
    for (i, s_i) in s.iter().enumerate() {
        cur_scores[i] += i + 1;
        for (j, c) in s_i.iter().enumerate() {
            if *c == 'o' {
                cur_scores[i] += a[j];
            }
        }
    }
    let cur_max = *cur_scores.iter().max().unwrap();

    for (i, &cur_score) in cur_scores.iter().enumerate() {
        let mut after_score = cur_score;
        let mut ans = 0;
        'outer: for (score, problems) in problem_vec.iter().enumerate().rev() {
            for &j in problems {
                if after_score >= cur_max {
                    break 'outer;
                }
                if s[i][j] != 'o' {
                    after_score += score * 100;
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
