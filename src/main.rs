mod header;
mod name_field;
use header::Header;

trait HeaderField {
    fn as_bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
    fn truncate(&self) -> Vec<u8>;
    fn pad(&self) -> Vec<u8>;
}

#[derive(Debug)]
struct Archive {
    header: Header,
    file: Vec<u8>
}

fn main() {
    println!("hi");
}
