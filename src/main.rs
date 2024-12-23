use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

// アセットのチェックを行う構造体
struct AssetChecker {
    name_pattern: Regex,             // ファイル名のパターン（例：^[a-z][a-z0-9_]*$）
    allowed_extensions: Vec<String>, // 許可する拡張子（例：["png", "jpg"]）
}

impl AssetChecker {
    // 新しいチェッカーを作成
    fn new(pattern: &str, extensions: Vec<String>) -> Self {
        AssetChecker {
            name_pattern: Regex::new(pattern).unwrap(),
            allowed_extensions: extensions,
        }
    }

    // ファイルをチェックするメソッド
    fn check_file(&self, path: &Path) -> Result<(), String> {
        // ファイル名を取得
        if let Some(file_name) = path.file_name() {
            let name_str = file_name.to_string_lossy();

            // 命名規則のチェック
            if !self.name_pattern.is_match(&name_str) {
                return Err(format!("Invalid file name: {}", name_str));
            }

            // 拡張子のチェック
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !self.allowed_extensions.contains(&ext_str.to_string()) {
                    return Err(format!("Invalid extension: {}", ext_str));
                }
            }
        }
        Ok(())
    }
}

fn main() {
    // チェッカーの初期化（例：小文字とアンダースコアのみを許可）
    let checker = AssetChecker::new(
        r"^[a-z][a-z0-9_]*$",
        vec!["png".to_string(), "jpg".to_string()],
    );

    // カレントディレクトリ以下のファイルを再帰的にチェック
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            match checker.check_file(path) {
                Ok(()) => println!("OK: {}", path.display()),
                Err(e) => println!("Error: {} for file {}", e, path.display()),
            }
        }
    }
}
