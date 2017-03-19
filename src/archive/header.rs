extern crate byteorder;

use std::io::Cursor;
use archive::header::byteorder::{BigEndian, ReadBytesExt};
use std::iter::FromIterator;

#[derive(Clone,Debug)]
pub struct Header{
    file_name: String,
    size: i32
}

#[derive(Clone,Debug)]
pub struct Headers{
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

impl FromIterator<u8> for Headers {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        let mut headers = Headers{ headers: Vec::new() };
        let mut header_bytes = Vec::new();

        for b in iter {
            header_bytes.push(b);
        }

        let null_char = "\0";
        let name = String::from_utf8(header_bytes[0..99].to_vec()).unwrap().trim_right_matches(null_char).clone().to_string();
        let size = Cursor::new(header_bytes[124..135].to_vec()).read_i32::<BigEndian>().unwrap();

        let header = Header { file_name: name.to_string(), size: size };
        headers.push(header);

        headers
    }
}
