use regex::Regex;
use same_file::Handle;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

use crate::components::CustomError;

/// Check if file is being read and written at the same time.
fn check_same_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let path_to_read = Path::new(file_path);
    let handle = Handle::from_path(path_to_read)?;

    let stdout_handle = Handle::stdout()?;
    if stdout_handle == handle {
        //     return Box::new(Err(io::Error::new(
        //         ErrorKind::Other,
        //         "reading and writing to the same file",
        //     )));
        // }
        return Err(CustomError::new("reading and writing to the same file"));
    }
    Ok(())
}

/// Read content from file.
pub fn read_from_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    check_same_file(file_path)?;
    let file = File::open(Path::new(file_path))?;

    let mut file = BufReader::new(file);
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Write content to file.
pub fn write_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    check_same_file(file_path)?;

    let mut file = File::create(Path::new(file_path))?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// Copy a directory from src to dest.
fn copy_directory(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    if !src.is_dir() {
        return Err(CustomError::new("source is not a directory"));
    }

    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry_path.file_name().unwrap());

        if entry_path.is_dir() {
            copy_directory(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

/// Copy a file from src to dest.
fn copy_file(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    if !src.is_file() {
        return Err(CustomError::new("source is not a directory"));
    }

    if !dest.parent().unwrap().exists() {
        fs::create_dir_all(dest.parent().unwrap())?;
    }

    fs::copy(&src, &dest)?;
    Ok(())
}

/// Backup from origin(src) to dest.
pub fn backup_from_origin(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    let file_name = match src_path.file_name() {
        Some(value) => value.to_os_string(),
        None => OsString::from(""),
    };

    let binding = dest_path.join(file_name);
    let dest_path = binding.as_path();

    // Avoid backing up modified files
    if dest_path.exists() {
        return Ok(());
    }

    if src_path.is_dir() {
        copy_directory(src_path, dest_path)?;
    } else if src_path.is_file() {
        copy_file(src_path, dest_path)?;
    } else {
        let message = format!("source `{:?}`is not a directory or file", src_path);
        return Err(CustomError::new(&message));
    }
    Ok(())
}

/// Rollback src to origin(dest).
pub fn rollback_to_origin(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    let file_name = match dest_path.file_name() {
        Some(value) => value.to_os_string(),
        None => OsString::from(""),
    };

    let binding = src_path.join(file_name);
    let src_path = binding.as_path();

    if !src_path.exists() {
        return Ok(());
    }

    if dest_path.is_dir() {
        copy_directory(src_path, dest_path)?;
    } else if dest_path.is_file() {
        copy_file(src_path, dest_path)?;
    } else {
        let message = format!("destination `{:?}`is not a directory or file", dest_path);
        return Err(CustomError::new(&message));
    }
    Ok(())
}

/// Replace the content of file to `to` according to the `from`(regex pattern).
pub fn replace_regex(file_path: &str, from: &str, to: &str) -> Result<(), Box<dyn Error>> {
    let file_content = read_from_file(file_path)?;

    let pattern = Regex::new(from).unwrap();
    if !pattern.is_match(file_content.as_str()) {
        return Err(CustomError::new("regex not match!"));
    }
    let modify_contnet = pattern.replace_all(file_content.as_str(), to).to_string();

    match write_to_file(file_path, &modify_contnet) {
        Ok(()) => Ok(()),
        Err(err) => {
            return Err(CustomError::new(
                format!("write content failed, {}", err).as_str(),
            ));
        }
    }
}

/// Delete the content of file according to the `from`(regex pattern).
pub fn delete_regex(file_path: &str, from: &str) -> Result<(), Box<dyn Error>> {
    match replace_regex(file_path, from, "") {
        Ok(()) => Ok(()),
        Err(err) => {
            return Err(CustomError::new(format!("{}", err).as_str()));
        }
    }
}

/// Join a file name to path.
pub fn join_path(hades_path: &str, file_name: &'static str) -> String {
    let path = Path::new(&hades_path);
    path.join(file_name).to_string_lossy().to_string()
}
