mod serialize;

use std::fs;
use std::io::ErrorKind;
use serde_json;
use serde;
use std::path;


fn main() {
    println!("Reading json file...");
    let path = path::Path::new("C:\\Users\\17579\\Development\\nerv\\api\\file.download.json");

    let file_info = serialize::serialize::decode(path);

    file_info.print();
}