pub mod header;

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
    let h = header::Header{
        name: String::from("asd")
    };
    println!("h: {:?}", h);
    println!("Hello world!");
}
