use super::{CurrentDirError, FileData};
use multimap::MultiMap;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;

#[cfg(all(not(test), not(feature = "benchmarking")))]
fn run_cache_on_interval(filecache: Arc<FileCache>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

        // Update filecache in intervals
        loop {
            interval.tick().await;
            filecache.update_memory_cache();
            let _ = filecache.update_filecache_file_from_memory();
        }
    });
}

pub struct FileCache {
    file_location: PathBuf,
    cache: Mutex<MultiMap<String, FileData>>,
}

impl FileCache {
    pub fn new(location: PathBuf) -> Result<Arc<Self>, CurrentDirError> {
        // Check that file can be opened, otherwise try to create new file
        let filecache = Arc::new(FileCache {
            file_location: location.clone(),
            cache: Mutex::new(MultiMap::new()),
        });

        if !filecache.check_file_parses() {
            let _ = File::create(&location).map_err(|_| CurrentDirError::CannotReadDir {
                dir_name: location.to_string_lossy().to_string(),
            })?;
            filecache.update_filecache_file_with_new()?;
        }

        filecache.read_cache_from_cache_file()?;

        #[cfg(all(not(test), not(feature = "benchmarking")))]
        run_cache_on_interval(Arc::clone(&filecache));
        Ok(filecache)
    }

    pub fn find_file(&self, name: &str) -> Option<Vec<FileData>> {
        let cache = self.cache.lock().unwrap();
        Some(cache.get_vec(&name.to_lowercase())?.to_vec())
    }

    /// This function is expensive, gets called when creating a new instance of this struct.
    fn read_cache_from_cache_file(&self) -> Result<(), CurrentDirError> {
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
                Ok(data) => Ok((data.name.to_lowercase(), data)),
                Err(_error) => Err(CurrentDirError::CannotSerialize),
            })
            .collect::<Result<MultiMap<String, FileData>, CurrentDirError>>()?;

        let mut cache = self.cache.lock().unwrap();
        *cache = new_data;

        Ok(())
    }

    pub fn update_memory_cache(&self) {
        let new_data = WalkDir::new("/")
            .into_iter()
            .filter_map(|e| e.ok())
            .map(FileData::from)
            .map(|f| (f.name.clone().to_lowercase(), f))
            .collect::<MultiMap<String, FileData>>();

        let mut cache = self.cache.lock().unwrap();
        *cache = new_data;
    }

    /// Function to check that cache file is formatted properly
    pub fn check_file_parses(&self) -> bool {
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

    /// This function is expensive, gets called when running filecache for the fist time
    fn update_filecache_file_with_new(&self) -> Result<(), CurrentDirError> {
        let tmp_file_path = Path::new("cache/cache.tmp");
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

    pub fn update_filecache_file_from_memory(&self) -> Result<(), CurrentDirError> {
        let tmp_file_path = Path::new("cache/cache.tmp");
        let tempfile: File =
            File::create(tmp_file_path).map_err(|_| CurrentDirError::CannotCreateFile)?;
        let buf_writer = BufWriter::new(tempfile);
        let mut csv_writer = csv::Writer::from_writer(buf_writer);

        let cache = self.cache.lock().unwrap();
        for element in cache.iter_all().flat_map(|(_, v)| v) {
            csv_writer
                .serialize(element)
                .map_err(|_| CurrentDirError::CannotReadDir {
                    dir_name: self.file_location.to_string_lossy().to_string(),
                })?;
        }

        std::fs::rename(tmp_file_path, &self.file_location)
            .map_err(|_| CurrentDirError::CannotCreateFile)?;
        Ok(())
    }
}
