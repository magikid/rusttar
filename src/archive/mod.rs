mod header;
use archive::header::*;

struct Archive {
    header: Header,
    file: Vec<u8>
}
