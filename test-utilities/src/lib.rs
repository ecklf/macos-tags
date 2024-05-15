use std::{
    fs::{remove_file, File},
    io, panic,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub fn generate_test_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_test_file() -> Result<PathBuf, io::Error> {
    let id = generate_test_id();
    let path = format!("/tmp/{}", id);
    File::create(&path)?;

    Ok(Path::new(&path).to_owned())
}

pub fn delete_test_file(path: &PathBuf) -> Result<(), io::Error> {
    remove_file(path)
}

pub fn run_file_test<T>(test: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: FnOnce(&Path) -> Result<(), Box<dyn std::error::Error>> + panic::UnwindSafe,
{
    let path = generate_test_file()?;
    let catch_unwind = panic::catch_unwind(|| test(&path.as_path()));
    let result = catch_unwind;
    delete_test_file(&path)?;

    assert!(result.is_ok());
    Ok(())
}
