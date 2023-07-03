use super::{CurrentDirError, FileData};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileCache {
    file_location: PathBuf,
}

impl FileCache {
    pub fn new(location: PathBuf) -> Result<Self, CurrentDirError> {
        // Check that file can be opened otherwise try to create new file
        if location.exists() && File::open(&location).is_ok() {
            return Ok(FileCache {
                file_location: location,
            });
        }
        let _ = File::create(&location).map_err(|_| CurrentDirError::CannotReadDir {
            dir_name: location.to_string_lossy().to_string(),
        })?;

        Ok(FileCache {
            file_location: location,
        })
    }

    pub fn find_file(&self, name: &str) -> Result<Option<Vec<FileData>>, CurrentDirError> {
        let file = File::options()
            .read(true)
            .open(&self.file_location)
            .map_err(|_| CurrentDirError::CannotReadDir {
                dir_name: self.file_location.to_string_lossy().to_string(),
            })?;

        let buf_reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(buf_reader);

        let data = csv_reader
            .deserialize::<FileData>()
            .map(|f| f.map_err(|_| CurrentDirError::CannotSerialize))
            .filter(|data| data.is_err() || data.as_ref().is_ok_and(|f| f.name.eq(name)))
            .collect::<Result<Vec<FileData>, CurrentDirError>>()?;

        if data.is_empty() {
            return Ok(None);
        }

        Ok(Some(data))
    }

    /// Function to check that file is formatted properly
    pub fn check_file_parses(&self) -> bool {
        self.find_file("something").is_ok()
    }

    pub fn update_filecache(&self) -> Result<(), CurrentDirError> {
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
    fn test_file_caching() {
        let fc = super::FileCache::new(std::path::Path::new("asd").to_path_buf()).unwrap();
        fc.update_filecache().unwrap();
    }

    #[test]
    #[ignore]
    fn test_finding_file() {
        let fc = super::FileCache::new(std::path::Path::new("asd").to_path_buf()).unwrap();
        let data = fc.find_file("asd").unwrap();
        println!("data: {:?}", data);
        panic!()
    }
}