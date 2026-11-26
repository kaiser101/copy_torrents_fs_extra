#[allow(unused_imports)]
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

use clap::Parser;
#[allow(unused_imports)]
use std::env;

use anyhow::Result;
use log::{info, warn};

mod helper;

use helper::{
    copy_files_parcopy, get_available_space, init_log, log_folder_size, move_recursively_fs_extra,
    move_recursively_fs_extra_with_progress,
};

#[derive(Parser, Debug)]
#[command(name = "file_mover", version, about)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "E:/Administrator/Downloads/Code samples/Node/torrent_client_js/torrents"
    )]
    src: String,

    #[arg(short, long, default_value = "K:/Movies/New")]
    dest: String,

    #[arg(short, long, default_value_t = 1)]
    method: i32,
}

fn main() -> Result<()> {
    init_log();

    let args = Args::parse();
    let src = args.src;
    let dest = args.dest;
    let method = args.method;

    if !(Path::new(&src).exists() && Path::new(&dest).exists()) {
        warn!("Either source: {src} or destination: {dest} does not exist, exiting");
        log::logger().flush();
        return Ok(());
    }

    let folder_size = log_folder_size(&src)?;

    let available_space = get_available_space(&dest)?;

    if folder_size > available_space {
        warn!("Not enough space available on disk, exiting!");
        log::logger().flush();
        return Ok(());
    }

    let now = SystemTime::now();

    match method {
        1 => move_recursively_fs_extra(&src, &dest)?,
        2 => move_recursively_fs_extra_with_progress(&src, &dest)?,
        3 => copy_files_parcopy(&src, &dest)?,
        _ => {
            info!("Not implemented");
        }
    }

    let secs = now.elapsed()?.as_secs();
    info!("Files copied in {secs} seconds");

    log::logger().flush();
    Ok(())
}
