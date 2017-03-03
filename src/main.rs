mod header;
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;
use header::pad_header;

#[derive(Debug)]
enum Typefield{
  REGTYPE = 0x30,
  AREGTYPE = 0x00,
  LNKTYPE = 0x31,
  SYMTYPE = 0x32,
  CHRTYPE = 0x33,
  BLKTYPE = 0x34,
  DIRTYPE = 0x35,
  FIFOTYPE = 0x36,
  CONTTYPE = 0x37
}

fn main() {
    let mut file = BufWriter::new(File::create("test.tar").unwrap());
    let name = pad_header("hi".to_string(), 100);
    file.write(name.as_slice());
    println!("Hello world!");
}
