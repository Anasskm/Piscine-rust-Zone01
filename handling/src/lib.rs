use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file_handle = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
    {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    if let Err(err) = file_handle.write_all(content.as_bytes()) {
        panic!("{}", err);
    }
}
