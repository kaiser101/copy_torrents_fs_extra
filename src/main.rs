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
    get_available_space, init_log, log_folder_size, move_recursively_fs_extra,
    move_recursively_fs_extra_with_progress,
};

#[derive(Parser, Debug)]
#[command(name = "file_mover", version, about)]
struct Args {
    src: String,
    dest: String,
    #[arg(short, long, default_value_t = 2)]
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

    let folder_size = log_folder_size(&src);

    let available_space = get_available_space(&dest);

    if folder_size > available_space {
        warn!("Not enough space available on disk, exiting!");
        log::logger().flush();
        return Ok(());
    }

    let now = SystemTime::now();

    match method {
        1 => move_recursively_fs_extra_with_progress(&src, &dest)?,
        2 => move_recursively_fs_extra(&src, &dest)?,
        _ => {
            info!("Not implemented");
        }
    }

    let secs = now.elapsed()?.as_secs();
    info!("Files copied in {secs} seconds");

    log::logger().flush();
    Ok(())
}
