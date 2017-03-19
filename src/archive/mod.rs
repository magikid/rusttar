mod header;

use std::fs::File;
use std::io::Read;
use archive::header::*;

pub struct Archive {
    pub file: String,
}

impl Archive {
    pub fn listing(&self) -> Vec<String>{
        println!("{:?}", self.parse_headers());
        vec![self.file.clone()]
    }

    fn parse_headers(&self) -> Headers {
        let file_handle = self.open_file();
        file_handle.bytes()
                   .take(512)
                   .map(|x| x.unwrap())
                   .collect()
    }

    fn open_file(&self) -> File {
        match File::open(self.file.clone()) {
            Ok(f) => f,
            Err(f) => panic!{ format!("Problem opening file {}", f.to_string()) }
        }
    }
}
