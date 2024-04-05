mod sqlite;

pub trait Database {
    fn init(&self);
}

pub fn get_database(config: &str) -> Box<dyn Database> {
    match config {
        "sqlite" => Box::new(sqlite::SqliteDatabase),
        _ => panic!("Unsupported database type"),
    }
}
