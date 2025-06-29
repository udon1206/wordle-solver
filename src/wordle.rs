use std::collections::{HashMap, HashSet};

const POW3: [usize; 5] = [1, 3, 9, 27, 81];

/// Wordle のフィードバックパターンを数値化
pub fn pattern_code(answer: &str, guess: &str) -> usize {
    let a = answer.as_bytes();
    let g = guess.as_bytes();
    let mut code = 0;
    let mut counts = [0u8; 26]; // a〜z の出現残数カウント

    // 1) 緑判定：一致なら即コード加算、そうでなければ counts にカウント
    for i in 0..5 {
        if g[i] == a[i] {
            // green = 2
            code += 2 * POW3[i];
        } else {
            counts[(a[i] - b'a') as usize] += 1;
        }
    }

    // 2) 黄判定：非緑の位置だけ、残カウントがあればコード加算
    for i in 0..5 {
        if g[i] != a[i] {
            let idx = (g[i] - b'a') as usize;
            if counts[idx] > 0 {
                // yellow = 1
                code += POW3[i];
                counts[idx] -= 1;
            }
        }
    }

    code
}

/// 情報利得 (エントロピー)
pub fn information_gain(cands: &[String], guess: &str) -> f64 {
    let total = cands.len() as f64;
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for ans in cands {
        let pat = pattern_code(ans, guess);
        *freq.entry(pat).or_default() += 1;
    }
    let mut h_f = 0.0;
    for &count in freq.values() {
        let p = count as f64 / total;
        h_f -= p * p.log2();
    }
    h_f
}

/// ベストな推測単語を決める
pub fn best_guess(cands: &[String], all_guesses: &[String]) -> String {
    let cands_set: HashSet<&String> = cands.iter().collect();
    let mut igs: Vec<(String, f64)> = all_guesses
        .iter()
        .map(|g| (g.clone(), information_gain(cands, g)))
        .collect();
    igs.sort_by(|a, b| match b.1.partial_cmp(&a.1).unwrap() {
        std::cmp::Ordering::Equal => {
            let a_in = cands_set.contains(&a.0);
            let b_in = cands_set.contains(&b.0);
            match (a_in, b_in) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        }
        other => other,
    });
    igs.into_iter().next().unwrap().0
}

/// 次の最適な文字列を提案
pub fn propose_optimal_string(
    answer_strings: &[String],
    answer_statuses: &[String],
    cands: &[String],
    all_guesses: &[String],
) -> String {
    assert_eq!(
        answer_strings.len(),
        answer_statuses.len(),
        "答えの文字列とステータスの長さが一致しません"
    );
    if answer_strings.is_empty() {
        return "tarse".to_string();
    }
    let mut candidates = cands.to_vec();
    for (answer, status) in answer_strings.iter().zip(answer_statuses.iter()) {
        let mut code = 0;
        for (i, ch) in status.chars().enumerate() {
            code += (ch.to_digit(10).unwrap() as usize) * POW3[i];
        }
        candidates.retain(|w| pattern_code(w, answer) == code);
    }
    best_guess(&candidates, all_guesses)
}
