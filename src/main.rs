use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

use std::env;

use log::{error, info, warn};

mod helper;

use helper::{
    get_available_space, init_log, log_folder_size, move_recursively_fs_extra,
    move_recursively_fs_extra_with_progress,
};

fn main() -> Result<(), Box<dyn Error>> {
    init_log();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        warn!("Not enough arguments provided");
        log::logger().flush();

        panic!("Usage: cargo run -- <source folder> <destination folder>");
    }

    let [_, src, dest, method, ..] = args.as_slice() else {
        warn!("Not enough arguments provided");
        todo!()
    };

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

    match method.as_str() {
        "1" => match move_recursively_fs_extra_with_progress(src, dest) {
            Ok(_) => info!("Files moved"),
            Err(e) => error!("Error: {e:?}"),
        },
        "2" => match move_recursively_fs_extra(src, dest) {
            Ok(_) => info!("Files moved"),
            Err(e) => error!("Error: {e:?}"),
        },
        &_ => {
            println!("Not implemented");
        }
    }

    match now.elapsed() {
        Ok(elapsed) => {
            let sec = elapsed.as_secs();
            info!("Files copied in {sec} seconds");
        }
        Err(e) => {
            error!("Error: {e:?}");
        }
    }

    log::logger().flush();
    Ok(())
}
