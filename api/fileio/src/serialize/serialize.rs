use std::{fmt, fs};
use std::io::ErrorKind;

#[derive(serde::Deserialize, Debug)]
pub struct FileMetadata {
    version: String,
    metadata: std::collections::HashMap<String, FileData>,
}

impl FileMetadata {
    pub fn print(&self){
        for (k, v) in self.metadata.iter() {
            println!("[{}]: \n{}", k, v);
        }

    }
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "\tfile: {}\n\tid: {}\n\trlkey: {}\n\tdtype: {}\n\ttelescope: {}\n\tsize: {}\n\tmode: {}\n", self.file, self.id, self.rlkey, self.dtype, self.telescope, self.size, self.mode)
    }
}

#[derive(serde::Deserialize, Debug)]
struct FileData {
    file: String,
    id: String,
    rlkey: String,
    dtype: String,
    telescope: String,
    size: String,
    mode: String,
}


pub fn decode(path: &std::path::Path)->FileMetadata{
    let data = fs::read_to_string(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("File not found at {}", path.display());
        } else {
            panic!("Error reading file {error:?}");
        }
    });

    let file_info: FileMetadata = serde_json::from_str(&data).expect("There was an error while deserializing json");
    return file_info;

}