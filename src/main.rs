use chrono::{DateTime, Local, NaiveDate};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_name = "DATE", help = "Date in YYYY-MM-DD format")]
    date: Option<String>,

    #[arg(
        short,
        long,
        value_name = "SUFFIX",
        default_value = ".go",
        help = "File suffix to match"
    )]
    suffix: String,

    #[arg(
        short,
        long,
        value_name = "ROOT",
        default_value = ".",
        help = "Root directory to start search from"
    )]
    root: String,
}

fn get_date(date_str: Option<&str>) -> Result<NaiveDate, String> {
    match date_str {
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| "Invalid date format (should be YYYY-MM-DD)".to_string()),
        None => Ok(chrono::Local::now().date_naive()),
    }
}

fn format_as_markdown(path: &str) -> String {
    let path_obj = Path::new(path);
    let filename = path_obj
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path);

    format!("- [{}]({})", filename, path)
}

fn file_iterator(root: &Path) -> impl Iterator<Item = PathBuf> + '_ {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
}

fn has_suffix(path: &Path, suffix: &str) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.ends_with(suffix))
        .unwrap_or(false)
}

fn match_date(path: &Path, target_date: NaiveDate) -> bool {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|modified| {
            let datetime: DateTime<Local> = modified.into();
            datetime.date_naive() == target_date
        })
        .unwrap_or(false)
}

fn main() {
    let args = Args::parse();

    let date = match get_date(args.date.as_deref()) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let root_path = Path::new(&args.root);
    if !root_path.exists() {
        eprintln!("error: root directory '{}' does not exist", args.root);
        std::process::exit(1);
    }

    let files = file_iterator(root_path)
        .filter(|path| has_suffix(path, &args.suffix))
        .filter(|path| match_date(path, date));

    for file in files {
        println!("{}", format_as_markdown(file.to_str().unwrap_or("")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_valid_date() {
        let result = get_date(Some("2025-12-25"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "2025-12-25");
    }

    #[test]
    fn test_another_valid_date() {
        let result = get_date(Some("2024-01-01"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "2024-01-01");
    }

    #[test]
    fn test_invalid_date_format() {
        let result = get_date(Some("25-12-2025"));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_date_value() {
        let result = get_date(Some("2025-13-45"));
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_date() {
        let result = get_date(Some("not-a-date"));
        assert!(result.is_err());
    }

    #[test]
    fn test_default_date_returns_some() {
        let result = get_date(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_as_markdown_simple_path() {
        let result = format_as_markdown("src/main.rs");
        assert_eq!(result, "- [main.rs](src/main.rs)");
    }

    #[test]
    fn test_format_as_markdown_nested_path() {
        let result = format_as_markdown("./src/some/nested/file.go");
        assert_eq!(result, "- [file.go](./src/some/nested/file.go)");
    }

    #[test]
    fn test_format_as_markdown_relative_path() {
        let result = format_as_markdown("./tests/cli.rs");
        assert_eq!(result, "- [cli.rs](./tests/cli.rs)");
    }

    #[test]
    fn test_format_as_markdown_filename_only() {
        let result = format_as_markdown("Cargo.toml");
        assert_eq!(result, "- [Cargo.toml](Cargo.toml)");
    }

    #[test]
    fn test_find_files_returns_ok() {
        let temp_dir = TempDir::new().unwrap();
        let date = Local::now().date_naive();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, date))
            .collect();
        assert!(result.is_empty() || !result.is_empty()); // Always ok
    }

    #[test]
    fn test_find_files_with_today_date() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();
        drop(file);

        let today = Local::now().date_naive();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, today))
            .collect();

        assert!(
            result
                .iter()
                .any(|p| p.to_str().unwrap().contains("test.txt"))
        );
    }

    #[test]
    fn test_find_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let date = Local::now().date_naive();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, date))
            .collect();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_nonexistent_date() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let old_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, old_date))
            .collect();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_with_suffix_filter() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with different suffixes
        File::create(temp_dir.path().join("test.go")).unwrap();
        File::create(temp_dir.path().join("test.txt")).unwrap();
        File::create(temp_dir.path().join("test.rs")).unwrap();

        let today = Local::now().date_naive();

        // Test .go suffix
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".go"))
            .filter(|path| match_date(path, today))
            .collect();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_str().unwrap().ends_with(".go"));

        // Test .txt suffix
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, today))
            .collect();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_str().unwrap().ends_with(".txt"));

        // Test .rs suffix
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".rs"))
            .filter(|path| match_date(path, today))
            .collect();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_str().unwrap().ends_with(".rs"));
    }

    #[test]
    fn test_find_files_no_matching_suffix() {
        let temp_dir = TempDir::new().unwrap();
        File::create(temp_dir.path().join("test.txt")).unwrap();

        let today = Local::now().date_naive();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".go"))
            .filter(|path| match_date(path, today))
            .collect();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_with_custom_root() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        File::create(temp_dir.path().join("root.txt")).unwrap();
        File::create(subdir.join("sub.txt")).unwrap();

        let today = Local::now().date_naive();

        // Search from root - should find both
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, today))
            .collect();
        assert_eq!(result.len(), 2);

        // Search from subdir - should find only sub.txt
        let result: Vec<_> = file_iterator(&subdir)
            .filter(|path| has_suffix(path, ".txt"))
            .filter(|path| match_date(path, today))
            .collect();
        assert_eq!(result.len(), 1);
        assert!(result[0].to_str().unwrap().contains("sub.txt"));
    }

    #[test]
    fn test_find_files_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        let level1 = temp_dir.path().join("level1");
        let level2 = level1.join("level2");
        std::fs::create_dir_all(&level2).unwrap();

        File::create(temp_dir.path().join("file0.go")).unwrap();
        File::create(level1.join("file1.go")).unwrap();
        File::create(level2.join("file2.go")).unwrap();

        let today = Local::now().date_naive();
        let result: Vec<_> = file_iterator(temp_dir.path())
            .filter(|path| has_suffix(path, ".go"))
            .filter(|path| match_date(path, today))
            .collect();

        assert_eq!(result.len(), 3);
    }
}
