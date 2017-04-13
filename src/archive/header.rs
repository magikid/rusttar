extern crate byteorder;

use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct Header{
    file_name: String,
    size: i32
}

#[derive(Clone,Debug)]
struct Headers{
    headers: Vec<Header>
}

impl Iterator for Headers {
    type Item = Header;
    fn next(&mut self) -> Option<Header> {
        self.headers.clone().into_iter().next()
    }
}

impl Headers {
    fn push(&mut self, elem: Header){
        self.headers.push(elem)
    }
}

enum HeaderOffsets{
    name = 0,
    mode = 100,
    uid = 108,
    gid = 116,
    size = 124,
    mtime = 136
}

impl Header {
    pub fn new(bytes: Vec<u8>) -> Header {
        let file_name = get_name(bytes[(HeaderOffsets::name as usize)..(HeaderOffsets::mode as usize)].to_vec());
        let file_size = get_size(bytes[(HeaderOffsets::size as usize)..(HeaderOffsets::mtime as usize)].to_vec());
        let header = Header { file_name: file_name, size: file_size };
        header
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

fn get_name(bytes: Vec<u8>) -> String{
    match String::from_utf8(bytes) {
        Ok(x) => trim_null_chars(x),
        Err(_) => String::from("")
    }
}

fn get_size(bytes: Vec<u8>) -> i32{
    // For some reason, GNU tar writes the file size as a string instead of a number so we
    // first parse it as a string, then parse the number from the string.
    let size_string = match String::from_utf8(bytes) {
        Ok(x) => trim_null_chars(x),
        Err(_) => panic!("No size from string")
    };

    let size = match i32::from_str_radix(&size_string, 8) {
        Ok(x) => x,
        Err(_) => 0
    };
    size
}

fn trim_null_chars(cstr: String) -> String{
    cstr.trim_right_matches("\0").to_string()
}
