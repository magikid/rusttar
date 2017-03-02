use std::fmt;

#[derive(Debug)]
pub struct Header{
    name:     String
    /*
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
    */
}

impl fmt::Binary for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.name.as_bytes())
    }
}
