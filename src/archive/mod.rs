/*
    Copyright (C) 2017 Chris Jones

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

mod header;

use std::fs::File;
use std::io::Read;
use archive::header::*;

pub struct Archive {
    pub file: String,
}

impl Archive {
    pub fn listing(&self) -> Vec<String> {
        let file_handle = self.open_file();
        let mut file_names = Vec::new();
        let mut file_bytes = file_handle
            .bytes()
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>();

        while !file_bytes.is_empty() {
            let first_byte = match file_bytes.first() {
                Some(x) => x.clone(),
                None => panic!("No byte?"),
            };
            if first_byte != 0u8 {
                let header_bytes = file_bytes.drain(..512).collect::<Vec<u8>>();
                let header = Header::new(header_bytes);
                file_names.push(header.file_name());

                let skip_length = calculate_blocks(header.size());
                if skip_length * 512 > file_bytes.len() as i32 {
                    file_bytes.drain(..);
                } else {
                    file_bytes.drain(..(skip_length * 512) as usize);
                }
            } else {
                file_bytes.drain(..512);
                continue;
            }
        }
        file_names
    }

    fn open_file(&self) -> File {
        match File::open(self.file.clone()) {
            Ok(f) => f,
            Err(f) => panic!{ format!("Problem opening file {}", f.to_string()) },
        }
    }
}

fn calculate_blocks(size: i32) -> i32 {
    ((size as f64 / 512f64) as f64).ceil() as i32
}
