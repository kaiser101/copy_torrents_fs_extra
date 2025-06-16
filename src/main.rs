#![allow(unused_variables)]
use std::time::SystemTime;

use fs_extra::dir::{copy_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Error;

use lms::core::copy;
use lms::parse::Flag;
use std::io::stdin;
use std::io::Error as StdError;

fn main() -> Result<(), Error> {
    let mut src = String::new();
    let mut dest = String::new();

    println!("Enter source directory");
    stdin().read_line(&mut src)?;

    println!("Enter destination directory");
    stdin().read_line(&mut dest)?;

    let now = SystemTime::now();

    copy_recursively_fs_extra(&src.trim(), &dest.trim())?;

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Files copied in {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }

    Ok(())
}

pub fn copy_recursively_fs_extra(src: &str, dest: &str) -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: 4096000,
        ..Default::default()
    };

    let handle = |process_info: TransitProcess| TransitProcessResult::ContinueOrAbort;

    copy_with_progress(src, dest, &options, handle)?;

    Ok(())
}

pub fn lms_copy(src: &str, dest: &str) -> Result<(), StdError> {
    let _ = copy(src, dest, Flag::from_bits_truncate(7))?;

    Ok(())
}
