pub mod filecache;

use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct CurrentDir {
    path: PathBuf,
    file_cache: Arc<filecache::FileCache>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderData {
    pub name: String,
    pub files: Vec<FileData>,
    pub is_at_root: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FileData {
    name: String,
    path: PathBuf,
    filetype: FileType,
}

impl FileData {
    fn new(name: &str, path: &Path, filetype: FileType) -> FileData {
        FileData {
            name: name.to_owned(),
            path: path.to_owned(),
            filetype,
        }
    }
}

impl Ord for FileData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.filetype, &self.name, &self.path).cmp(&(other.filetype, &other.name, &other.path))
    }
}

impl PartialOrd for FileData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.filetype.partial_cmp(&other.filetype) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.path.partial_cmp(&other.path)
    }
}

impl From<walkdir::DirEntry> for FileData {
    fn from(value: walkdir::DirEntry) -> Self {
        FileData {
            name: value.file_name().to_string_lossy().to_string(),
            path: value.path().to_path_buf(),
            filetype: FileType::from(value.file_type()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Folder,
    File,
    Link,
}

impl From<std::fs::FileType> for FileType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_dir() {
            FileType::Folder
        } else if value.is_symlink() {
            FileType::Link
        } else {
            FileType::File
        }
    }
}

#[derive(Debug, Display, Serialize, Deserialize, Error)]
pub enum CurrentDirError {
    AlreadyAtRoot,
    PathCannotBeMadeAbsolute,
    CannotGetFileType,
    #[display(fmt = "Directory \"{}\" cannot be found", dir_name)]
    CannotReadDir {
        dir_name: String,
    },
    #[display(fmt = "Cannot move to directory {}", file_name)]
    CannotMoveToFile {
        file_name: String,
    },
    IsntUTF8,
    CannotSerialize,
    CannotCreateFile,
    CannotWriteToFile,
    SearchedFileNotFound,
}

impl CurrentDir {
    pub fn new(path: &Path) -> Self {
        let parsed_path = CurrentDir::parse_path_to_absolute(path).unwrap();
        CurrentDir {
            path: parsed_path,
            file_cache: filecache::FileCache::new(Path::new("cache/cache").to_path_buf()).unwrap(),
        }
    }

    /// Returns the old path as the ok type
    pub fn move_to_parent_dir(&mut self) -> Result<String, CurrentDirError> {
        let old_path = self
            .path
            .to_str()
            .ok_or(CurrentDirError::IsntUTF8)?
            .to_string();
        let parent_path = self.path.parent().ok_or(CurrentDirError::AlreadyAtRoot)?;
        self.path = parent_path.to_owned();
        Ok(old_path)
    }

    pub fn move_to_dir(&mut self, path: &Path, to_parent: bool) -> Result<(), CurrentDirError> {
        let parsed = CurrentDir::parse_path_to_absolute(path)?;
        if !parsed.is_dir() && !to_parent {
            return Err(CurrentDirError::CannotMoveToFile {
                file_name: path.to_str().unwrap().to_string(),
            });
        }
        self.path = parsed;

        if to_parent {
            self.move_to_parent_dir()?;
        }

        Ok(())
    }

    pub fn get_siblings(&self) -> Result<Vec<FileData>, CurrentDirError> {
        let mut siblings = vec![];
        for entry in fs::read_dir(&self.path)
            .map_err(|err| CurrentDirError::CannotReadDir {
                dir_name: err.to_string(),
            })?
            .filter_map(|v| v.ok())
        {
            let name = entry
                .file_name()
                .to_str()
                .ok_or(CurrentDirError::IsntUTF8)?
                .to_owned();
            let path = entry.path();
            let filetype = FileType::from(
                entry
                    .file_type()
                    .map_err(|_| CurrentDirError::CannotGetFileType)?,
            );

            siblings.push(FileData {
                name,
                path,
                filetype,
            })
        }
        Ok(siblings)
    }

    pub fn get_current_folder_name(&self) -> Result<&str, CurrentDirError> {
        self.path
            .file_name()
            .unwrap_or(self.path.as_os_str())
            .to_str()
            .ok_or(CurrentDirError::IsntUTF8)
    }

    pub fn get_folder_data(&self) -> Result<FolderData, CurrentDirError> {
        let siblings = self.get_siblings()?;
        let name = self.get_current_folder_name()?.to_owned();

        Ok(FolderData {
            name,
            files: siblings,
            is_at_root: self.current_dir_is_root(),
        })
    }

    pub fn parse_path_to_absolute(path: &Path) -> Result<PathBuf, CurrentDirError> {
        fs::canonicalize(path).map_err(|_| CurrentDirError::PathCannotBeMadeAbsolute)
    }

    pub fn current_dir_is_root(&self) -> bool {
        self.path.parent().is_none()
    }

    pub fn search_files(
        &self,
        name: &str,
        search_files: bool,
        search_folders: bool,
        search_links: bool,
    ) -> Result<Vec<FileData>, CurrentDirError> {
        Ok(self
            .file_cache
            .find_file(name)
            .ok_or(CurrentDirError::SearchedFileNotFound)?
            .into_iter()
            .filter(|file| match file.filetype {
                FileType::File if search_files => true,
                FileType::Folder if search_folders => true,
                FileType::Link if search_links => true,
                _ => false,
            })
            .collect::<Vec<FileData>>())
    }
}

#[cfg(test)]
mod tests {
    use super::{FileData, FileType};
    use std::path::Path;

    #[test]
    fn filedata_is_equal() {
        let some = FileData {
            name: String::from("asd"),
            path: Path::new("/").to_owned(),
            filetype: FileType::File,
        };
        assert_eq!(some, some);
    }

    #[test]
    fn filedata_is_not_equal() {
        let some = FileData {
            name: String::from("asd"),
            path: Path::new("/").to_owned(),
            filetype: FileType::File,
        };
        let different_filetype = FileData {
            name: String::from("asd"),
            path: Path::new("/").to_owned(),
            filetype: FileType::Folder,
        };
        let different_name = FileData {
            name: String::from("asdf"),
            path: Path::new("/").to_owned(),
            filetype: FileType::File,
        };
        assert_ne!(some, different_filetype);
        assert_ne!(some, different_name);
    }

    #[test]
    fn filedata_sorts_name_and_filetype_correctly() {
        let model = vec![
            FileData::new("a", Path::new("asd"), FileType::Folder),
            FileData::new("b", Path::new("asd"), FileType::Folder),
            FileData::new("a", Path::new("asd"), FileType::File),
            FileData::new("b", Path::new("asd"), FileType::File),
            FileData::new("a", Path::new("asd"), FileType::Link),
            FileData::new("b", Path::new("asd"), FileType::Link),
        ];

        let mut randomized = vec![
            FileData::new("a", Path::new("asd"), FileType::File),
            FileData::new("b", Path::new("asd"), FileType::Folder),
            FileData::new("b", Path::new("asd"), FileType::File),
            FileData::new("b", Path::new("asd"), FileType::Link),
            FileData::new("a", Path::new("asd"), FileType::Link),
            FileData::new("a", Path::new("asd"), FileType::Folder),
        ];

        randomized.sort_unstable();
        assert_eq!(model, randomized);
    }

    #[test]
    fn filetype_orders_correctly() {
        let mut randomized = vec![FileType::Link, FileType::Folder, FileType::File];

        let model = vec![FileType::Folder, FileType::File, FileType::Link];

        randomized.sort_unstable();

        assert_eq!(randomized, model)
    }
}
