mod header_field;
use archive::header::header_field::*;

#[derive(Debug)]
pub struct Header{
    pub name: NameField
}
