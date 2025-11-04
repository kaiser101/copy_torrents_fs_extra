use fs_extra::dir::{
    get_size, move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult,
};
use fs_extra::error::Error;

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;

use lms::core::copy;
use lms::parse::Flag;

use log::{debug, info, LevelFilter};

use std::io::Error as StdError;

pub fn init_log() {
    fast_log::init(
        Config::new()
            .level(LevelFilter::Info)
            .chan_len(Some(100000))
            .console()
            .file_split(
                "target/logs/",
                Rolling::new(RollingType::BySize(LogSize::KB(500))),
                KeepType::KeepNum(2),
                LogPacker {},
            ),
    )
    .unwrap();
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

#[allow(dead_code)]
pub fn lms_copy(src: &str, dest: &str) -> Result<(), StdError> {
    let _ = copy(src, dest, Flag::from_bits_truncate(7))?;

    Ok(())
}

pub fn log_folder_size(src: &str) -> f32 {
    let folder_size = get_size(src).expect("Could not read folder");
    let size_in_gb = folder_size / (1024 * 1024 * 1024);
    info!("{:.2}", size_in_gb);

    size_in_gb as f32
}
