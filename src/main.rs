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
}

fn get_date(date_str: Option<&str>) -> Result<NaiveDate, String> {
    match date_str {
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| "Invalid date format. Use YYYY-MM-DD".to_string()),
        None => Ok(chrono::Local::now().date_naive()),
    }
}

fn find_files_by_date(start_path: &Path, target_date: NaiveDate) -> Result<Vec<String>, String> {
    let mut matching_files = Vec::new();

    for entry in WalkDir::new(start_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(metadata) = fs::metadata(entry.path()) {
                if let Ok(modified) = metadata.modified() {
                    let datetime: DateTime<Local> = modified.into();
                    let file_date = datetime.date_naive();

                    if file_date == target_date {
                        if let Some(path_str) = entry.path().to_str() {
                            matching_files.push(path_str.to_string());
                        }
                    }
                }
            }
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

    match find_files_by_date(Path::new("."), date) {
        Ok(files) => {
            for file in files {
                println!("{}", file);
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
    fn test_find_files_returns_ok() {
        let temp_dir = TempDir::new().unwrap();
        let date = Local::now().date_naive();
        let result = find_files_by_date(temp_dir.path(), date);
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
        let result = find_files_by_date(temp_dir.path(), today).unwrap();

        assert!(result.iter().any(|p| p.contains("test.txt")));
    }

    #[test]
    fn test_find_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let date = Local::now().date_naive();
        let result = find_files_by_date(temp_dir.path(), date).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_nonexistent_date() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let old_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let result = find_files_by_date(temp_dir.path(), old_date).unwrap();

        assert_eq!(result.len(), 0);
    }
}
