use proconio::input;

fn main() {
    let mut field = vec![];
    for _ in 0..3 {
        input! {
            row: [i64; 3],
        }
        field.push(row);
    }

    let mut state = vec![vec![-1_i64; 3]; 3];
    let winner = rec(&field, &mut state, 0);

    println!("{}", if winner == 0 { "Takahashi" } else { "Aoki" });
}

fn rec(field: &Vec<Vec<i64>>, state: &mut Vec<Vec<i64>>, cur: i64) -> i64 {
    if check_rows(state, cur) {
        return cur;
    }
    if check_rows(state, cur ^ 1) {
        return cur ^ 1;
    }
    if check_cols(state, cur) {
        return cur;
    }
    if check_cols(state, cur ^ 1) {
        return cur ^ 1;
    }
    if check_diag(state, cur) {
        return cur;
    }
    if check_diag(state, cur ^ 1) {
        return cur ^ 1;
    }
    if all_filled(state) {
        return if calc_score(field, state, cur) > calc_score(field, state, cur ^ 1) {
            cur
        } else {
            cur ^ 1
        };
    }

    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] != -1 {
                continue;
            }
            state[i][j] = cur;
            let res = rec(field, state, cur ^ 1);
            state[i][j] = -1;
            if res == cur {
                // 一つでも勝てる手があれば、その手を選ぶ
                return cur;
            }
        }
    }

    // 一つも勝てる手がない場合、相手の勝ち
    cur ^ 1
}

fn check_rows(state: &Vec<Vec<i64>>, player: i64) -> bool {
    let mut ok = false;
    for i in 0..3 {
        if state[i].iter().all(|&x| x == player) {
            ok = true;
        }
    }
    ok
}

fn check_cols(state: &Vec<Vec<i64>>, player: i64) -> bool {
    let mut ok = false;
    for i in 0..3 {
        if state.iter().all(|x| x[i] == player) {
            ok = true;
        }
    }
    ok
}

fn check_diag(state: &Vec<Vec<i64>>, player: i64) -> bool {
    let mut ok = false;
    if (0..3).all(|i| state[i][i] == player) {
        ok = true;
    }
    if (0..3).all(|i| state[i][2 - i] == player) {
        ok = true;
    }
    ok
}

fn all_filled(state: &Vec<Vec<i64>>) -> bool {
    state.iter().all(|row| row.iter().all(|&x| x != -1))
}

fn calc_score(field: &Vec<Vec<i64>>, state: &mut Vec<Vec<i64>>, cur: i64) -> i64 {
    let mut score = 0;
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == cur {
                score += field[i][j];
            }
        }
    }
    score
}
