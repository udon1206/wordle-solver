use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// アプリケーション全体で共有する状態
#[derive(Clone)]
pub struct AppState {
    pub solutions: Vec<String>,
    pub guesses: Vec<String>,
}

/// ファイルから単語リストを読み込む
pub fn load_words(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let words = reader
        .lines()
        .filter_map(Result::ok)
        .map(|w| w.trim().to_lowercase())
        .filter(|w| w.len() == 5)
        .collect::<Vec<_>>();
    Ok(words)
}
