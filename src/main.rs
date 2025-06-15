#![allow(unused_variables)]
use std::time::SystemTime;

use fs_extra::dir::{move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Error;

use lms::core::copy;
use lms::parse::Flag;
use std::env;
use std::io::Error as StdError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = &args[1];
    let dest = &args[2];

    let now = SystemTime::now();

    match copy_recursively_fs_extra(src, dest) {
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

pub fn copy_recursively_fs_extra(src: &str, dest: &str) -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: 4096000,
        ..Default::default()
    };

    let handle = |process_info: TransitProcess| TransitProcessResult::ContinueOrAbort;

    move_dir_with_progress(src, dest, &options, handle)?;

    Ok(())
}

pub fn lms_copy(src: &str, dest: &str) -> Result<(), StdError> {
    let _ = copy(src, dest, Flag::from_bits_truncate(7))?;

    Ok(())
}
