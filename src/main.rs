use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

struct AssetChecker {
    name_pattern: Regex,
    allowed_extensions: Vec<String>,
}

impl AssetChecker {
    fn new(pattern: &str, extensions: Vec<String>) -> Self {
        AssetChecker {
            name_pattern: Regex::new(pattern).unwrap(),
            allowed_extensions: extensions,
        }
    }

    fn check_file(&self, path: &Path) -> Result<(), String> {
        if path.file_name().is_some() {
            // ファイル名から拡張子を除いた部分を取得
            let name_without_ext = path
                .file_stem()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();

            // 拡張子を除いたファイル名でパターンマッチング
            if !self.name_pattern.is_match(&name_without_ext) {
                return Err(format!(
                    "Invalid file name: {} (without extension)",
                    name_without_ext
                ));
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

    // 指定ディレクトリ以下のファイルを再帰的にチェック
    for entry in WalkDir::new("./checktarget")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            match checker.check_file(path) {
                Ok(()) => println!("OK: {}", path.display()),
                Err(e) => println!("Error: {} for file {}", e, path.display()),
            }
        }
    }
}
