use constcat::concat;
use dotenvy_macro::dotenv;

pub const DB_HOST: &str = dotenv!("DB_HOST", "localhost");
pub const DB_PORT: &str = dotenv!("DB_PORT", "5432");
pub const DB_NAME: &str = dotenv!("DB_NAME", "axum_backend_db");
pub const DB_USER: &str = dotenv!("DB_USER", "postgres");
pub const DB_PASSWORD: &str = dotenv!("DB_PASSWORD", "postgres");

pub const DB_CONFIG: &str = concat!(
    "host=",
    DB_HOST,
    " port=",
    DB_PORT,
    " user=",
    DB_USER,
    " password=",
    DB_PASSWORD,
    " dbname=",
    DB_NAME
);
