mod enigma;

//use log;
//use serde_json;
//use serde;
use std::path;


fn main() {
    println!(target: "runinfo", "Reading json file...");
    let path = path::Path::new("C:\\Users\\17579\\Development\\nerv\\api\\file.download.json");

    let file_info = enigma::json::decode(path);

    //file_info.print();
}