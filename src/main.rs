mod header;
use header::HeaderField;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let name = HeaderField{value: "foo".to_string().as_bytes().to_vec(), length: 100};
    let mut f = File::create("foo.txt").unwrap();
    f.write_all(&(*name.as_bytes())).unwrap();
    println!("{}", name);
}
