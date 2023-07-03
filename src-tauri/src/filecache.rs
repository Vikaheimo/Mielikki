use super::{CurrentDirError, FileData, FileType};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf}, sync::{Arc, Mutex},
};
use walkdir::WalkDir;
use multimap::MultiMap;

#[derive(Debug)]
pub struct CachedFile {
    path: PathBuf,
    filetype: FileType,
}

#[derive(Debug)]
pub struct FileCache {
    file_location: PathBuf,
    cache: Arc<Mutex<MultiMap<String, CachedFile>>>
}

impl CachedFile {
    pub fn from_filedata(data: FileData) -> (String, CachedFile) {
        (data.name, CachedFile{ path: data.path, filetype: data.filetype })
    }
}

impl FileCache {
    pub fn new(location: PathBuf) -> Result<Self, CurrentDirError> {
        // Check that file can be opened, otherwise try to create new file
        let fc = FileCache {
            file_location: location.clone(),
            cache: Arc::new(Mutex::new(MultiMap::new()))
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

    pub fn find_file(&self, name: &str) -> Result<Option<Vec<FileData>>, CurrentDirError> {
        unimplemented!()
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
            .map(|f| {
                match f {
                    Ok(data) => Ok(CachedFile::from_filedata(data)),
                    Err(_error) =>  Err(CurrentDirError::CannotSerialize)
                }
            }).collect::<Result<MultiMap<String, CachedFile>,CurrentDirError>>()?;
            
        let cache_lock = self.cache.clone();
        let mut data = cache_lock.lock().unwrap();
        *data = new_data;

        Ok(())
    }

    /// Function to check that file is formatted properly
    pub fn check_file_parses(&self) -> bool {
        self.find_file("something").is_ok()
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
        csv_writer.flush().map_err(|_| CurrentDirError::CannotWriteToFile)?;
        std::fs::rename(tmp_file_path, &self.file_location)
            .map_err(|_| CurrentDirError::CannotCreateFile)?;
        Ok(())
    }
}

mod test {
    #[test]
    #[ignore]
    fn test_finding_file() {
        let fc = super::FileCache::new(std::path::Path::new("asd").to_path_buf()).unwrap();
        let data = fc.find_file("asd").unwrap();
        println!("data: {:?}", data);
        panic!()
    }
}
