use crate::database::Database;

pub struct SqliteDatabase;

impl Database for SqliteDatabase {
    fn init(&self) {
        println!("Loading the Sqlite database...");
    }
}
