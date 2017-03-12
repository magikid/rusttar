struct Archive {
    file_entries: Vec<FileEntry>,
    footer: [u8; 1024]
}

#[derive(Debug)]
struct FileEntry {
    header: Header,
    file: Vec<u8>,
}

#[derive(Debug)]
struct StarHeader{
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    chksum: [u8; 8],
    typeflag: [u8; 1],
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 131],
    atime: [u8; 12],
    ctime: [u8; 12]
}

impl Header for StarHeader{
    fn as_bytes(&self) {
        self.name
    }
}

trait Header {
    fn as_bytes(&self);
}
