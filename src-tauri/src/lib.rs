use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct CurrentDir {
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderData {
    pub name: String,
    pub files: Vec<FileData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    name: String,
    path: PathBuf,
    filetype: FileType,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Error, Display, Serialize, Deserialize)]
pub enum CurrentDirError {
    AlreadyAtRoot,
    PathCannotBeMadeAbsolute,
    CannotReadDir,
    CannotMoveToFile,
    IsntUTF8,
}

impl CurrentDir {
    pub fn new(path: &Path) -> Self {
        let parsed_path = CurrentDir::parse_path_to_absolute(path).unwrap();
        CurrentDir { path: parsed_path }
    }

    pub fn move_to_parent_dir(&mut self) -> Result<(), CurrentDirError> {
        let parent_path = self.path.parent().ok_or(CurrentDirError::AlreadyAtRoot)?;
        self.path = parent_path.to_owned();
        Ok(())
    }

    pub fn move_to_dir(&mut self, path: &Path) -> Result<(), CurrentDirError> {
        let parsed = CurrentDir::parse_path_to_absolute(path)?;
        if !parsed.is_dir() {
            return Err(CurrentDirError::CannotMoveToFile);
        }
        self.path = parsed;
        Ok(())
    }

    pub fn get_siblings(&self) -> Result<Vec<FileData>, CurrentDirError> {
        let mut siblings = vec![];
        for entry in fs::read_dir(&self.path)
            .map_err(|_| CurrentDirError::CannotReadDir)?
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
                    .map_err(|_| CurrentDirError::CannotReadDir)?,
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
            .ok_or(CurrentDirError::CannotReadDir)?
            .to_str()
            .ok_or(CurrentDirError::IsntUTF8)
    }

    pub fn get_folder_data(&self) -> Result<FolderData, CurrentDirError> {
        let siblings = self.get_siblings()?;
        let name = self.get_current_folder_name()?.to_owned();

        Ok(FolderData {
            name,
            files: siblings,
        })
    }

    pub fn parse_path_to_absolute(path: &Path) -> Result<PathBuf, CurrentDirError> {
        fs::canonicalize(path).map_err(|_| CurrentDirError::PathCannotBeMadeAbsolute)
    }
}
