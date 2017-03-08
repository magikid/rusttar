mod header;
use header::Header;
use header::header_field::HeaderField;

fn main() {
    let h = Header{
        name: HeaderField{
            value: "foo".to_string().as_bytes().to_vec(), 
            length: 100
        }
    };
    println!("{:?}", h);
    println!("Hi!");
}
