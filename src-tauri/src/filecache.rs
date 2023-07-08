use super::FileData;
use std::path::Path;
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;
use rusqlite::backup;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedFile {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub filetype: String,
}

impl CachedFile {
    pub fn from_filedata(value: FileData, id: i32) -> Self {
        CachedFile {
            id,
            name: value.name,
            path: value.path.to_string_lossy().to_string(),
            filetype: value.filetype.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FileCache {
    database: Mutex<Connection>,
}
const DATABASE_FILE: &str = "mielikki.db";

impl FileCache {
    pub async fn new() -> Self {
        let first_time = !Path::new(DATABASE_FILE).exists();
        let connection = Connection::open_in_memory().await.unwrap();
        let cache: FileCache = FileCache {
            database: Mutex::new(connection),
        };

        if first_time {
            cache.create_cache_table().await.unwrap();
            cache.cache_all_files().await.unwrap();
        }

        cache
    }

    pub async fn find_file(&self, name: String) -> Option<Vec<FileData>> {
        let db = self.database.lock().await;
        let data = db
            .call(move |conn| {
                Ok({
                    let mut statement =
                        conn.prepare("SELECT * FROM file_cache WHERE name = '?1'")?;
                    let files = statement
                        .query_map([name.to_lowercase()], |row| {
                            Ok(CachedFile {
                                id: row.get(0)?,
                                name: row.get(1)?,
                                path: row.get(2)?,
                                filetype: row.get(3)?,
                            })
                        })?
                        .collect::<Result<Vec<CachedFile>, rusqlite::Error>>()?;
                    Ok::<_, rusqlite::Error>(files)
                })
            })
            .await
            .ok()?
            .ok()?;
        if data.is_empty() {
            return None;
        }

        data.iter()
            .map(FileData::try_from)
            .collect::<Result<Vec<FileData>, _>>()
            .ok()
    }

    /// Should be called only when initializing the database for the first time
    async fn cache_all_files(&self) -> Result<(), tokio_rusqlite::Error> {
        let db = self.database.lock().await;
        let t = std::time::Instant::now();
        db.call(move |conn| {
            for entry in walkdir::WalkDir::new("/")
                .into_iter()
                .filter_map(|e| e.ok())
                .map(FileData::from)
                .enumerate()
                .map(|(i, filedata)| CachedFile::from_filedata(filedata, i as i32))
            {
                conn.execute(
                    "INSERT INTO file_cache (id, name, filetype, path) VALUES (?1, ?2, ?3, ?4)",
                    (entry.id, entry.name, entry.path, entry.filetype),
                )?;
            }
            println!("{}", t.elapsed().as_secs());
            Ok(())
        })
        .await
    }

    async fn create_cache_table(&self) -> Result<usize, tokio_rusqlite::Error> {
        self.database
            .lock()
            .await
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
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{filecache::CachedFile, FileData};

    #[test]
    fn cachedfile_from_filedata() {
        let fd = FileData {
            name: String::from("test"),
            path: Path::new("/test/path").to_owned(),
            filetype: crate::FileType::File,
        };

        assert_eq!(
            CachedFile::from_filedata(fd, 1),
            CachedFile {
                id: 1,
                name: String::from("test"),
                path: String::from("/test/path"),
                filetype: String::from("File"),
            }
        )
    }
}
