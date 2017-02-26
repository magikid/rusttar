use std::str;

struct Header{
  name:     [u8; 100],
  mode:     [u8; 8],
  uid:      [u8; 8],
  gid:      [u8; 8],
  size:     [u8; 12],
  mtime:    [u8; 12],
  chksum:   [u8; 8],
  typeflag: [u8; 1],
  linkname: [u8; 100],
  magic:    [u8; 6],
  version:  [u8; 2],
  uname:    [u8; 32],
  gname:    [u8; 32],
  devmajor: [u8; 8],
  devminor: [u8; 8],
  prefix:   [u8; 155]
}

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
  println!("Hello world!");
}
