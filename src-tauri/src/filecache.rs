use super::FileData;
use rusqlite::{backup, named_params};
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

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

fn cache_files_on_interval(cache: Arc<FileCache>) {
    tokio::spawn(async move {
        let secondary_cache = FileCache::create_secondary().await;
        loop {
            // TODO, handle errors
            secondary_cache.cache_all_files().await.unwrap();
            secondary_cache.backup_database_to_file().await.unwrap();
            cache.read_database_from_backup().await.unwrap();
        }
    });
}

#[derive(Debug)]
pub struct FileCache {
    database: Mutex<Connection>,
}
const DATABASE_FILE: &str = "mielikki.db";

impl FileCache {
    pub async fn new() -> Arc<Self> {
        let first_time = !Path::new(DATABASE_FILE).exists();
        let connection = Connection::open_in_memory().await.unwrap();
        let cache: FileCache = FileCache {
            database: Mutex::new(connection),
        };

        if first_time {
            cache.create_cache_table().await.unwrap();
            cache.cache_all_files().await.unwrap();
            cache.backup_database_to_file().await.unwrap();
        } else {
            cache.read_database_from_backup().await.unwrap();
        }
        let data = Arc::new(cache);
        cache_files_on_interval(Arc::clone(&data));
        data
    }

    pub async fn create_secondary() -> Self {
        let connection = Connection::open_in_memory().await.unwrap();
        let cache: FileCache = FileCache {
            database: Mutex::new(connection),
        };
        cache.create_cache_table().await.unwrap();
        cache
    }

    pub async fn find_file(&self, name: String, exact: bool) -> Option<Vec<FileData>> {
        let db = self.database.lock().await;
        let search = name.clone();
        let data = db
            .call(move |conn| {
                Ok({
                    let mut statement = conn.prepare(
                        "SELECT id, name, path, filetype FROM file_cache WHERE name LIKE :name",
                    )?;
                    let files = statement
                        .query_map(named_params! {":name": format!("{}%", search)}, |row| {
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
            .unwrap()
            .unwrap();
        if data.is_empty() {
            return None;
        }

        let asd = data
            .iter()
            .filter(|file| !exact || file.name.eq(&name))
            .map(FileData::try_from)
            .collect::<Result<Vec<FileData>, _>>();
        println!("Data: {:?}", &asd);
        asd.ok()
    }

    /// Should be called only when initializing the database for the first time
    async fn cache_all_files(&self) -> Result<(), tokio_rusqlite::Error> {
        self.clear_database().await?;
        self.create_cache_table().await?;
        let db = self.database.lock().await;

        db.call(move |conn| {
            for entry in walkdir::WalkDir::new("/")
                .into_iter()
                .filter_map(|e| e.ok())
                .map(FileData::from)
                .enumerate()
                .map(|(i, filedata)| CachedFile::from_filedata(filedata, i as i32))
            {
                conn.execute(
                    "INSERT INTO file_cache (id, name, path, filetype) VALUES (?1, ?2, ?3, ?4)",
                    (entry.id, entry.name, entry.path, entry.filetype),
                )?;
            }
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

    pub async fn backup_database_to_file(&self) -> Result<(), tokio_rusqlite::Error> {
        let src = self.database.lock().await;
        src.call(|memory_conn| {
            let mut backup_conn = rusqlite::Connection::open(DATABASE_FILE)?;
            let backup = backup::Backup::new(memory_conn, &mut backup_conn)?;
            backup.run_to_completion(100, std::time::Duration::from_millis(0), None)?;

            Ok(())
        })
        .await?;

        Ok(())
    }

    pub async fn read_database_from_backup(&self) -> Result<(), tokio_rusqlite::Error> {
        let db = self.database.lock().await;
        db.call(|memory_conn| {
            let backup_conn = rusqlite::Connection::open(DATABASE_FILE)?;
            let backup = backup::Backup::new(&backup_conn, memory_conn)?;
            backup.run_to_completion(100, std::time::Duration::from_millis(0), None)?;

            Ok(())
        })
        .await?;

        Ok(())
    }

    async fn clear_database(&self) -> Result<(), tokio_rusqlite::Error> {
        let db = self.database.lock().await;

        db.call(|conn| {
            conn.execute("DROP TABLE file_cache", [])?;

            Ok(())
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
