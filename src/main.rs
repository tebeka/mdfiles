use chrono::NaiveDate;
use clap::Parser;

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

fn main() {
    let args = Args::parse();

    match get_date(args.date.as_deref()) {
        Ok(date) => println!("{}", date),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
