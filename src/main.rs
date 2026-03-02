use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

use std::env;

use log::{error, info, warn};

mod helper;

use helper::{copy_recursively_fs_extra, init_log, log_folder_size};

fn main() -> Result<(), Box<dyn Error>> {
    init_log();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        warn!("Not enough arguments provided");
        log::logger().flush();

        panic!("Usage: cargo run -- <source folder> <destination folder>");
    }

    let src = &args[1];
    let dest = &args[2];

    if !(Path::new(&src).exists() && Path::new(&dest).exists()) {
        warn!(
            "Either source or destination {}/{} does not exist, exiting",
            src, dest
        );
        log::logger().flush();
        return Ok(());
    }

    log_folder_size(&src);

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
    Ok(())
}
