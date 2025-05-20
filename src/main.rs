#![allow(unused_variables)]
use fs_extra::dir::{move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Error;

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
        buffer_size: 1024000,
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
