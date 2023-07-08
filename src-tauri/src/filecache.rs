use super::{CurrentDirError, FileData};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

pub struct FileCache {
    database: Mutex<Connection>,
}
const DATABSE_FILE: &'static str = "mielikki.db";

impl FileCache {
    pub async fn new(location: PathBuf) -> Self {
        let first_time = !Path::new(DATABSE_FILE).exists();
        let connection = Connection::open(DATABSE_FILE).await.unwrap();

        if first_time {
            connection
                .call(|conn| {
                    conn.execute(
                        "CREATE TABLE file_cache (
                        id          INTEGER PRIMARY KEY,
                        name        TEXT NOT NULL,
                        filetype    TEXT NOT NULL,
                        path        TEXT NOT NULL
                    )",
                        [],
                    )
                })
                .await
                .unwrap();
        }
        FileCache {
            database: Mutex::new(connection),
        }
    }

    pub fn find_file(&self, name: &str) -> Option<Vec<FileData>> {
        unimplemented!()
    }

    pub fn cache_all_files(&self) {
        unimplemented!()
    }
}
