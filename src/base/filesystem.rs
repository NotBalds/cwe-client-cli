pub use std::fs::{create_dir_all as mkAllDirs, remove_dir_all as rmDirAll};
use {
    crate::base::{self, config},
    std::{
        fs::{self, File},
        io::prelude::*,
        path::{Path, PathBuf},
    },
};

pub fn new_path(path: &str) -> PathBuf {
    config::path().join(path)
}

pub fn exist(path: &str) -> bool {
    let path = new_path(path);
    if !path.exists() {
        return false;
    }
    true
}

pub fn exist_abs(path: String) -> bool {
    let path = PathBuf::from(path);
    if !path.exists() {
        return false;
    }
    true
}

pub fn echo(s: String, path: &PathBuf) {
    let mut f = File::create(path).expect(&format!("Can't create file {}", path.display()));
    f.write_all(s.as_bytes())
        .expect(&format!("Can't write data to file {}", path.display()));
}

pub fn becho(s: Vec<u8>, path: &PathBuf) {
    let mut f = File::create(path).expect(&format!("Can't create file {}", path.display()));
    f.write_all(&s)
        .expect(&format!("Can't write data to file {}", path.display()));
}

pub fn cat(path: &Path) -> String {
    let mut f = File::open(path).expect(&format!("Can't read file {}", path.display()));

    let mut s = String::new();
    let data = match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(err) => {
            base::log(&err.to_string(), 1);
            String::new()
        }
    };
    data
}

pub fn bcat(path: String) -> Vec<u8> {
    let mut file = File::open(path.clone()).expect(&format!("Can't open file {}", path));
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(err) => base::log(&format!("Can't read file {}: {}", path, err), 1),
    };

    buffer
}

pub fn ls(path: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let path = new_path(path);

    if path.exists() && path.is_dir() {
        let raw_entries = fs::read_dir(path).expect("Could not read directory");
        for entry in raw_entries {
            let entry = entry
                .expect("Could not read directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned();
            println!("{}", entry);
            entries.push(entry);
        }
    } else {
        base::log(&format!("Directory {} not found", path.display()), 1);
    }

    entries
}

pub fn cat_lines(path: &Path) -> Vec<String> {
    cat(path).lines().map(String::from).collect()
}

pub fn get_file_name(path: String) -> String {
    Path::new(&path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn del_file(path: &str) {
    let path = &new_path(path);
    if path.exists() {
        fs::remove_file(path).expect(&format!("Can't delete file {}", path.display()));
    }
}
