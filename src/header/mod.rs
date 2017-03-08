pub mod header_field;
use header::header_field::HeaderField;

#[derive(Debug)]
pub struct Header{
    pub name: HeaderField
}
