extern crate byteorder;

use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct Header{
    pub file_name: String,
    pub size: i32
}

#[derive(Clone,Debug)]
pub struct Headers{
    pub headers: Vec<Header>
}

impl Iterator for Headers {
    type Item = Header;
    fn next(&mut self) -> Option<Header> {
        self.headers.clone().into_iter().next()
    }
}

impl Headers {
    pub fn push(&mut self, elem: Header){
        self.headers.push(elem)
    }
}
