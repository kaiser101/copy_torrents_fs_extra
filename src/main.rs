#![allow(unused_variables)]
use fs_extra::dir::{move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Error;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    match copy_recursively_fs_extra() {
        Ok(_) => println!("Files moved"),
        Err(e) => println!("Error {}", e),
    }
}

pub fn copy_recursively_fs_extra() -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: 256000,
        ..Default::default()
    };

    let handle = |process_info: TransitProcess| TransitProcessResult::ContinueOrAbort;

    move_dir_with_progress(
        "E:/Administrator/Downloads/Code samples/Node/torrent_client_js/torrents",
        "I:/Movies/New",
        &options,
        handle,
    )?;
    Ok(())
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
            fs::remove_dir(entry.path())?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
