use super::{CurrentDirError, FileData, FileType};
use multimap::MultiMap;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::Mutex,
};
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Eq)]
pub struct CachedFile {
    pub path: PathBuf,
    pub filetype: FileType,
}

#[derive(Debug)]
pub struct FileCache {
    file_location: PathBuf,
    cache: Mutex<MultiMap<String, CachedFile>>,
}

impl CachedFile {
    pub fn from_filedata(data: FileData) -> (String, CachedFile) {
        (
            data.name,
            CachedFile {
                path: data.path,
                filetype: data.filetype,
            },
        )
    }
}

impl FileCache {
    pub fn new(location: PathBuf) -> Result<Self, CurrentDirError> {
        // Check that file can be opened, otherwise try to create new file
        let fc = FileCache {
            file_location: location.clone(),
            cache: Mutex::new(MultiMap::new()),
        };

        if fc.check_file_parses() {
            let _ = File::create(&location).map_err(|_| CurrentDirError::CannotReadDir {
                dir_name: location.to_string_lossy().to_string(),
            })?;
        }
        fc.update_filecache_file()?;
        fc.update_memory_cache_from_file()?;
        Ok(fc)
    }

    pub fn find_file(&self, name: &str) -> Option<Vec<FileData>> {
        let cache = self.cache.lock().unwrap();
        Some(
            cache
                .get_vec(name)?
                .iter()
                .map(|f| FileData::from_cachedfile_with_string(f, name.to_owned()))
                .collect(),
        )
    }
    pub fn update_memory_cache_from_file(&self) -> Result<(), CurrentDirError> {
        let file = File::options()
            .read(true)
            .open(&self.file_location)
            .map_err(|_| CurrentDirError::CannotReadDir {
                dir_name: self.file_location.to_string_lossy().to_string(),
            })?;

        let buf_reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(buf_reader);
        let new_data = csv_reader
            .deserialize::<FileData>()
            .map(|f| match f {
                Ok(data) => Ok(CachedFile::from_filedata(data)),
                Err(_error) => Err(CurrentDirError::CannotSerialize),
            })
            .collect::<Result<MultiMap<String, CachedFile>, CurrentDirError>>()?;

        let mut data = self.cache.lock().unwrap();
        *data = new_data;

        Ok(())
    }

    /// Function to check that file is formatted properly
    fn check_file_parses(&self) -> bool {
        let file = File::options()
            .read(true)
            .open(&self.file_location)
            .map_err(|_| CurrentDirError::CannotReadDir {
                dir_name: self.file_location.to_string_lossy().to_string(),
            });
        if file.is_err() {
            return false;
        }

        let buf_reader = BufReader::new(file.unwrap());
        let mut csv_reader = csv::Reader::from_reader(buf_reader);
        for entry in csv_reader.deserialize::<FileData>() {
            if entry.is_err() {
                return false;
            }
        }
        true
    }

    pub fn update_filecache_file(&self) -> Result<(), CurrentDirError> {
        let tmp_file_path = Path::new("cache.tmp");
        let tempfile =
            File::create(tmp_file_path).map_err(|_| CurrentDirError::CannotCreateFile)?;

        let buf_writer = BufWriter::new(tempfile);
        let mut csv_writer = csv::Writer::from_writer(buf_writer);

        for entry in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
            let filedata = FileData::from(entry);
            csv_writer
                .serialize(filedata)
                .map_err(|_| CurrentDirError::CannotReadDir {
                    dir_name: self.file_location.to_string_lossy().to_string(),
                })?
        }
        csv_writer
            .flush()
            .map_err(|_| CurrentDirError::CannotWriteToFile)?;
        std::fs::rename(tmp_file_path, &self.file_location)
            .map_err(|_| CurrentDirError::CannotCreateFile)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{CachedFile, FileData, FileType};
    use std::path::Path;

    #[test]
    fn cachedfile_from_fildata() {
        let fd = FileData {
            name: "test".to_owned(),
            path: Path::new("test").to_owned(),
            filetype: FileType::Folder,
        };
        let model = (
            "test".to_owned(),
            CachedFile {
                path: Path::new("test").to_owned(),
                filetype: FileType::Folder,
            },
        );
        assert_eq!(model, CachedFile::from_filedata(fd));
    }
}
