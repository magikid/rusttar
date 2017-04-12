mod header;

use std::fs::File;
use std::io::Read;
use archive::header::*;
use std::str::FromStr;

pub struct Archive {
    pub file: String,
}

impl Archive {
    pub fn listing(&self) -> Vec<String>{
        println!("{:?}", self.parse_headers());
        vec![self.file.clone()]
    }

    fn parse_headers(&self) -> Headers {
        let record_size = 512;
        let mut headers = Headers{ headers: Vec::new() };
        let file_handle = self.open_file();
        let mut header_bytes = file_handle.bytes()
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>();

        let mut base = 0;
        println!("base: {:?}, header_bytes: {:?}", base, header_bytes.len());
        while base + 1024 <= header_bytes.len() {
            let null_char = "\0";
            let name_str = match String::from_utf8(header_bytes[base..base+99].to_vec()) {
                Ok(x) => x,
                Err(_) => continue
            };
            let name = name_str.trim_right_matches(null_char).clone().to_string();
            let size = match i32::from_str(&String::from_utf8(header_bytes[base+124..base+135].to_vec()).unwrap()) {
                Ok(x) => x,
                Err(_) => record_size
            };

            let header = Header { file_name: name.to_string(), size: size };
            headers.push(header);
            let skip_files = record_size * ((size as f32 / record_size as f32) as f32).ceil() as i32;
            base += (record_size + skip_files) as usize;
            println!("base: {:?}, ceil: {:?}", base, skip_files);
        }

        headers
    }

    fn open_file(&self) -> File {
        match File::open(self.file.clone()) {
            Ok(f) => f,
            Err(f) => panic!{ format!("Problem opening file {}", f.to_string()) }
        }
    }
}
