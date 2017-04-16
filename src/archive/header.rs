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

extern crate byteorder;

#[derive(Clone,Debug)]
pub struct Header {
    file_name: String,
    size: i32,
}

enum HeaderOffsets {
    Name = 0,
    Mode = 100,
    Uid = 108,
    Gid = 116,
    Size = 124,
    Mtime = 136,
}

impl Header {
    pub fn new(bytes: Vec<u8>) -> Header {
        let file_name = get_name(bytes[(HeaderOffsets::Name as usize)..
                                 (HeaderOffsets::Mode as usize)]
                                         .to_vec());
        let file_size = get_size(bytes[(HeaderOffsets::Size as usize)..
                                 (HeaderOffsets::Mtime as usize)]
                                         .to_vec());
        let header = Header {
            file_name: file_name,
            size: file_size,
        };
        header
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

fn get_name(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(x) => trim_null_chars(x),
        Err(_) => String::from(""),
    }
}

fn get_size(bytes: Vec<u8>) -> i32 {
    // For some reason, GNU tar writes the file size as a string instead of a number so we
    // first parse it as a string, then parse the number from the string.
    let size_string = match String::from_utf8(bytes) {
        Ok(x) => trim_null_chars(x),
        Err(_) => panic!("No size from string"),
    };

    let size = match i32::from_str_radix(&size_string, 8) {
        Ok(x) => x,
        Err(_) => 0,
    };
    size
}

fn trim_null_chars(cstr: String) -> String {
    cstr.trim_right_matches("\0").to_string()
}
