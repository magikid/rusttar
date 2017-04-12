mod header;

use std::fs::File;
use std::io::Read;
use archive::header::*;
use std::str::FromStr;

pub struct Archive {
    pub file: String
}

impl Archive {
    pub fn listing(&self) -> Vec<String>{
        let file_handle = self.open_file();
        let mut file_names = Vec::new();
        let mut file_bytes = file_handle.bytes()
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>();

        let mut base = 0;
        while !file_bytes.is_empty() {
            let first_byte = match file_bytes.first() {
                Some(x) => x.clone(),
                None => panic!("No byte?")
            };
            if first_byte != 0u8 {
                let header_bytes = file_bytes.drain(..512).collect::<Vec<u8>>();
                let header = Header::new(header_bytes);
                file_names.push(header.file_name());

                let skip_length = calculate_blocks(header.size());
                if skip_length * 512 > file_bytes.len() as i32{
                    file_bytes.drain(..);
                } else {
                    file_bytes.drain(..(skip_length * 512) as usize);
                }
            }else{
                file_bytes.drain(..512);
                continue
            }
        }
        file_names
    }

    fn open_file(&self) -> File {
        match File::open(self.file.clone()) {
            Ok(f) => f,
            Err(f) => panic!{ format!("Problem opening file {}", f.to_string()) }
        }
    }
}

fn calculate_blocks(size: i32) -> i32 {
    ((size as f64 / 512f64) as f64).ceil() as i32
}

