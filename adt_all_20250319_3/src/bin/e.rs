use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let mut max_points = 0;
    for (j, s_i) in s.iter().enumerate() {
        let mut points = j + 1;
        for (i, &a_i) in a.iter().enumerate() {
            if s_i[i] == 'o' {
                points += a_i;
            }
        }
        max_points = max_points.max(points);
    }

    for (i, s_i) in s.iter().enumerate() {
        let mut points = i + 1;
        let mut unsolved = vec![];
        for (j, &a_j) in a.iter().enumerate() {
            if s_i[j] == 'o' {
                points += a_j;
            } else {
                unsolved.push(a_j);
            }
        }
        unsolved.sort();

        let mut ans = 0;
        while points < max_points {
            if let Some(a_j) = unsolved.pop() {
                points += a_j;
                ans += 1;
            } else {
                break;
            }
        }

        println!("{}", ans);
    }
}
