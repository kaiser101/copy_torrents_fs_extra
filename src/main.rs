#![allow(unused_variables)]
use std::time::SystemTime;

use fs_extra::dir::{move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Error;

use lms::core::copy;
use lms::parse::Flag;
use std::io::Error as StdError;

fn main() {
    let now = SystemTime::now();

    match move_dir_with_progress() {
        Ok(_) => println!("Files moved"),
        Err(e) => println!("Error {}", e),
    }

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Files copied in {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

pub fn copy_recursively_fs_extra() -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: 4096000,
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

pub fn lms_copy() -> Result<(), StdError> {
    let _ = copy(
        "E:/Administrator/Downloads/Code samples/Node/torrent_client_js/torrents",
        "I:/Movies/New",
        Flag::from_bits_truncate(7),
    )?;

    Ok(())
}
