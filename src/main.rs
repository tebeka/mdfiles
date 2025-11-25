use chrono::{DateTime, Local, NaiveDate};
use clap::Parser;
use std::fs;
use std::path::Path;
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
            .map_err(|_| "Invalid date format. Use YYYY-MM-DD".to_string()),
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

fn find_files(
    start_path: &Path,
    target_date: NaiveDate,
    suffix: &str,
) -> Result<Vec<String>, String> {
    let mut matching_files = Vec::new();

    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        // Check if file has the required suffix
        let Some(file_name) = path.file_name() else {
            continue;
        };
        let Some(name_str) = file_name.to_str() else {
            continue;
        };
        if !name_str.ends_with(suffix) {
            continue;
        }

        // Check modification date
        let Ok(metadata) = fs::metadata(path) else {
            continue;
        };
        let Ok(modified) = metadata.modified() else {
            continue;
        };

        let datetime: DateTime<Local> = modified.into();
        let file_date = datetime.date_naive();

        if file_date == target_date {
            let Some(path_str) = path.to_str() else {
                continue;
            };
            matching_files.push(path_str.to_string());
        }
    }

    Ok(matching_files)
}

fn main() {
    let args = Args::parse();

    let date = match get_date(args.date.as_deref()) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let root_path = Path::new(&args.root);
    if !root_path.exists() {
        eprintln!("Error: Root directory '{}' does not exist", args.root);
        std::process::exit(1);
    }

    match find_files(root_path, date, &args.suffix) {
        Ok(files) => {
            for file in files {
                println!("{}", format_as_markdown(&file));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
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
        let result = find_files(temp_dir.path(), date, ".txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_files_with_today_date() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();
        drop(file);

        let today = Local::now().date_naive();
        let result = find_files(temp_dir.path(), today, ".txt").unwrap();

        assert!(result.iter().any(|p| p.contains("test.txt")));
    }

    #[test]
    fn test_find_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let date = Local::now().date_naive();
        let result = find_files(temp_dir.path(), date, ".txt").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_nonexistent_date() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let old_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let result = find_files(temp_dir.path(), old_date, ".txt").unwrap();

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
        let result = find_files(temp_dir.path(), today, ".go").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with(".go"));

        // Test .txt suffix
        let result = find_files(temp_dir.path(), today, ".txt").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with(".txt"));

        // Test .rs suffix
        let result = find_files(temp_dir.path(), today, ".rs").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].ends_with(".rs"));
    }

    #[test]
    fn test_find_files_no_matching_suffix() {
        let temp_dir = TempDir::new().unwrap();
        File::create(temp_dir.path().join("test.txt")).unwrap();

        let today = Local::now().date_naive();
        let result = find_files(temp_dir.path(), today, ".go").unwrap();

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
        let result = find_files(temp_dir.path(), today, ".txt").unwrap();
        assert_eq!(result.len(), 2);

        // Search from subdir - should find only sub.txt
        let result = find_files(&subdir, today, ".txt").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("sub.txt"));
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
        let result = find_files(temp_dir.path(), today, ".go").unwrap();

        assert_eq!(result.len(), 3);
    }
}
