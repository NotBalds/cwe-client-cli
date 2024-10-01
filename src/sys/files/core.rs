pub mod path {
    use {
        directories::{BaseDirs, ProjectDirs},
        std::{
            fs,
            io::{Error, ErrorKind},
            path::PathBuf,
        },
    };

    pub struct Path {
        pub path: PathBuf,
    }
    impl Path {
        pub fn new(path: &str) -> Self {
            let home_dir = BaseDirs::new()
                .unwrap()
                .home_dir()
                .to_str()
                .unwrap()
                .to_string();
            let mut result = path.to_string();

            if result.starts_with('~') {
                result = result.replace("~", &home_dir);
            }

            let full_path = PathBuf::from(result);

            Path { path: full_path }
        }

        pub fn as_str(&self) -> &str {
            self.path.to_str().unwrap()
        }
    }

    pub struct AppPath {
        pub path: PathBuf,
    }
    impl AppPath {
        pub fn new(path: &str) -> Self {
            let base_path = ProjectDirs::from("su", "bald", "cwe-client")
                .expect("Failed to get ProjectDirs")
                .data_dir()
                .to_path_buf();

            let full_path = base_path.join(path);

            AppPath { path: full_path }
        }

        pub fn as_str(&self) -> &str {
            self.path.to_str().unwrap()
        }
    }

    pub fn ls(path: &PathBuf) -> Result<Vec<String>, Error> {
        if path.exists() && path.is_dir() {
            Ok(fs::read_dir(path)
                .unwrap()
                .map(|r| r.unwrap().file_name().to_str().unwrap().to_string())
                .collect())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("{} not found", path.display()),
            ))
        }
    }
}

pub mod create {
    use std::{
        fs::{self, File},
        io::Error,
        path::PathBuf,
    };

    pub fn file(path: &PathBuf) -> Result<File, Error> {
        File::create(&path)
    }
    pub fn dir(path: &PathBuf) -> Result<(), Error> {
        fs::create_dir(&path)
    }
}

pub mod delete {
    use std::{fs, io::Error, path::PathBuf};

    pub fn file(path: &PathBuf) -> Result<(), Error> {
        fs::remove_file(&path)
    }
    pub fn dir(path: &PathBuf) -> Result<(), Error> {
        fs::remove_dir(&path)
    }
}

pub mod write {
    use std::{fs, io::Error, path::PathBuf};

    pub fn bytes(path: &PathBuf, content: &[u8]) -> Result<(), Error> {
        fs::write(path, content)
    }
    pub fn str(path: &PathBuf, content: &str) -> Result<(), Error> {
        fs::write(path, content)
    }
}

pub mod read {
    use std::{fs, io::Error, path::PathBuf};

    pub fn bytes(path: &PathBuf) -> Result<Vec<u8>, Error> {
        fs::read(path)
    }
    pub fn string(path: &PathBuf) -> Result<String, Error> {
        fs::read_to_string(path)
    }
}
