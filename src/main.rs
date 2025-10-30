#![allow(unused_variables)]
use std::time::SystemTime;

use fs_extra::dir::{
    get_size, move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult,
};
use fs_extra::error::Error;

use fast_log::config::Config;
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;
use lms::core::copy;
use lms::parse::Flag;
use log::{debug, error, info, warn, LevelFilter};
use std::env;
use std::io::Error as StdError;

fn main() {
    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    fast_log::init(
        Config::new()
            .level(LevelFilter::Info)
            .chan_len(Some(100000))
            .console()
            .file_split(
                "target/logs/",
                Rolling::new(RollingType::ByDate(DateType::Day)),
                KeepType::KeepNum(2),
                LogPacker {},
            ),
    )
    .unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        warn!("Not enough arguments provided");
        log::logger().flush();

        panic!("Usage: cargo run -- <source folder> <destination folder>");
    }

    let src = &args[1];
    let dest = &args[2];

    let folder_size = get_size(src).expect("Could not read folder");
    let size_in_gb = folder_size / (1024 * 1024 * 1024);
    info!("{:.2}", size_in_gb);

    let now = SystemTime::now();

    match copy_recursively_fs_extra(src, dest) {
        Ok(_) => info!("Files moved"),
        Err(e) => error!("Error {}", e),
    }

    match now.elapsed() {
        Ok(elapsed) => {
            info!("Files copied in {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            error!("Error: {e:?}");
        }
    }

    log::logger().flush();
}

pub fn copy_recursively_fs_extra(src: &str, dest: &str) -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: 8192000,
        ..Default::default()
    };

    let handle = |process_info: TransitProcess| {
        debug!(
            "{0} out of {1} bytes copied, filename = {2}",
            process_info.copied_bytes, process_info.total_bytes, process_info.file_name,
        );

        TransitProcessResult::ContinueOrAbort
    };

    move_dir_with_progress(src, dest, &options, handle)?;

    Ok(())
}

pub fn lms_copy(src: &str, dest: &str) -> Result<(), StdError> {
    let _ = copy(src, dest, Flag::from_bits_truncate(7))?;

    Ok(())
}
