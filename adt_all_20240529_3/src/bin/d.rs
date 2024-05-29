use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut state = vec![0; 7];
    for (i, &c) in s.iter().enumerate() {
        match i + 1 {
            1 => state[3] += if c == '0' { 0 } else { 1 },
            2 => state[2] += if c == '0' { 0 } else { 1 },
            3 => state[4] += if c == '0' { 0 } else { 1 },
            4 => state[1] += if c == '0' { 0 } else { 1 },
            5 => state[3] += if c == '0' { 0 } else { 1 },
            6 => state[5] += if c == '0' { 0 } else { 1 },
            7 => state[0] += if c == '0' { 0 } else { 1 },
            8 => state[2] += if c == '0' { 0 } else { 1 },
            9 => state[4] += if c == '0' { 0 } else { 1 },
            10 => state[6] += if c == '0' { 0 } else { 1 },
            _ => unreachable!(),
        }
    }

    let mut cnt = 0;
    for i in 0..=6 {
        if cnt == 0 && state[i] > 0 {
            cnt += 1;
        }
        if cnt == 1 && state[i] == 0 {
            cnt += 1;
        }
        if cnt == 2 && state[i] > 0 {
            cnt += 1;
            break;
        }
    }

    println!("{}", if s[0] == '0' && cnt == 3 { "Yes" } else { "No" });
}
