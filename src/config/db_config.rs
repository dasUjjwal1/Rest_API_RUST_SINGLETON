use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}
impl Config {
    pub fn init() -> Config {
        let database_url = env::var("DATABASE_URL").expect("Error");
        Config { database_url }
    }
}
