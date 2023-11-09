use std::{
    fs,
    io::{self, Error},
    path::Path,
};
#[derive(Clone)]
pub struct Manager {
    folder: String,
}

impl Manager {
    pub fn new(folder: String) -> io::Result<Self> {
        let path = Path::new(&folder);
        if !path.exists() {
            return Err(Error::new(io::ErrorKind::NotFound, "Folder Does Not exist"));
        };
        Ok(Manager { folder })
    }
    /// List All Lua Files Inside The Folder
    pub fn list(&mut self) -> io::Result<Vec<String>> {
        let entries = fs::read_dir(&self.folder)?;
        let files = entries
            .filter(|e| {
                e.as_ref().unwrap().path().extension() == Some(std::ffi::OsStr::new("lua"))
                    && e.as_ref().unwrap().path().is_file()
            })
            .map(|luas| {
                luas.as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    
            });
        Ok(files.collect())
    }
}
