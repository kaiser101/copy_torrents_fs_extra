use fs_extra::dir::{
    get_size, move_dir, move_dir_with_progress, CopyOptions, TransitProcess, TransitProcessResult,
};
use fs_extra::error::Error;

use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;

use lms::core::copy;
use lms::parse::Flag;

use log::{info, LevelFilter};

use anyhow::Result;
use fs2::available_space;
use std::io::Error as StdError;
use std::path::Path;

use parcopy::{copy_dir, CopyOptions as OtherCopyOptions, Error as OtherError, OnConflict};

const BUFFER_SIZE: usize = 8 * 1024 * 1024;

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

pub fn move_recursively_fs_extra(src: &str, dest: &str) -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        ..Default::default()
    };

    move_dir(src, dest, &options)?;

    Ok(())
}

pub fn move_recursively_fs_extra_with_progress(src: &str, dest: &str) -> Result<(), Error> {
    let options = CopyOptions {
        overwrite: true,
        content_only: true,
        buffer_size: BUFFER_SIZE,
        ..Default::default()
    };

    let handle = |process: TransitProcess| {
        // Only print every ~100 MB to avoid I/O bottlenecks from stdout
        if process.copied_bytes % (500 * 1024 * 1024) < BUFFER_SIZE.try_into().unwrap() {
            info!(
                "Progress: {:.1}%",
                (process.copied_bytes as f64 / process.total_bytes as f64) * 100.0
            );
        }

        TransitProcessResult::ContinueOrAbort
    };

    move_dir_with_progress(src, dest, &options, handle)?;

    Ok(())
}

#[allow(dead_code)]
pub fn copy_files_parcopy(src: &str, dest: &str) -> Result<(), OtherError> {
    let options = OtherCopyOptions::default()
        .with_parallel(16) // Limit parallelism
        .with_on_conflict(OnConflict::Overwrite) // Overwrite existing
        .with_block_escaping_symlinks() // Block dangerous symlinks
        .without_fsync(); // Faster but less durable

    let stats = copy_dir(Path::new(src), Path::new(dest), &options)?;
    info!(
        "Copied {} files, skipped {}",
        stats.files_copied, stats.files_skipped
    );

    Ok(())
}

#[allow(dead_code)]
pub fn lms_copy(src: &str, dest: &str) -> Result<(), StdError> {
    let _ = copy(src, dest, Flag::from_bits_truncate(7))?;

    Ok(())
}

pub fn log_folder_size(src: &str) -> Result<f32> {
    let folder_size = get_size(src)? as f32;
    let one_gb = (1024 * 1024 * 1024) as f32;

    let size_in_gb: f32 = folder_size / one_gb;
    info!("{size_in_gb:.2}");

    Ok(size_in_gb)
}

pub fn get_available_space(dest: &str) -> Result<f32> {
    let path = Path::new(dest);

    let bytes = available_space(path)?;
    let gb = bytes as f32 / 1024.0 / 1024.0 / 1024.0;
    info!("Available space on disk {gb:.2} GB");

    Ok(gb)
}
