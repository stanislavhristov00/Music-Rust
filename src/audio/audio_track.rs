use std::{fs::File, io::{Error, ErrorKind}, path::Path};

fn get_base_name(path: &str) -> String {
    let path = Path::new(path);
    let filename = path.file_name().unwrap().to_str().unwrap();

    String::from(filename)
}

pub struct AudioTrack {
    full_path: String,
    basename: String,
    file_handle: Option<File>
}

// So we can use Vec::contains
impl PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path
    }
}

impl Default for AudioTrack {
    fn default() -> Self {
        AudioTrack {
            full_path: String::from(""),
            basename: String::from(""),
            file_handle: None
        }
    }
}

impl AudioTrack {
    pub fn new(path: &str) -> Result<AudioTrack, Error> {
        let file_handle = File::open(path);

        if let Err(e) = file_handle {
            eprintln!("Failed to open file {path}: {e}");
            return Err(e);
        }

        Ok(AudioTrack {
            full_path: String::from(path),
            basename: get_base_name(path),
            file_handle : Some(file_handle.unwrap())
        })
    }

    pub fn get_file_handle(&self) -> Result<File, Error> {
        if self.file_handle.is_some() {
            return self.file_handle.as_ref().unwrap().try_clone();
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "File handle is non existant"));
        }
    }

    pub fn get_base_name(&self) -> String {
        self.basename.clone()
    }

    pub fn clone(&self) -> Result<AudioTrack, Error> {
        let file_handle = self.get_file_handle();
        match file_handle {
            Err(e) => {
                Err(e)
            },
            Ok(handle) => {
                Ok(AudioTrack {
                    full_path: self.full_path.clone(),
                    basename: self.basename.clone(),
                    file_handle: Some(handle)
                })
            }
        }
    }
}