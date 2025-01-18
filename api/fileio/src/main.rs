mod enigma;

use std::path;
use log;

fn main() {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();
    log::info!("Opening json file ...");
    let path = path::Path::new("/export/home/ajax/jhoskins/Development/rust/nerv/api/file.download.json");

    log::info!("Decoding json file ...");
    log::warn!("Do you really want to crack the code?");
    //let file_info = enigma::json::decode(path);

    //file_info.print();
}