use super::{CurrentDirError, FileData};
use std::path::{Path, PathBuf};
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
            id: id,
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

    pub fn cache_all_files(&self) {
        unimplemented!()
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
