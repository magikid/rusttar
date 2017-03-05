use std::fmt;

struct HeaderField {
    value: Vec<u8>,
    length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes_to_write = pad_field(self.value, self.length);

        for byte in bytes_to_write.iter(){
            write!(f, "{:b}", byte);
        }
        write!(f, "{}", "")
    }
}

#[cfg(test)]
mod tests{
    use header::HeaderField;

    #[test]
    fn it_pads_the_length(){
        let foobar_field = "foobar".to_string().as_bytes().to_vec();
        let h = HeaderField { value: foobar_field, length: 100 };
        let output = format!("{}", h);
        assert_eq!(output.len(), h.length);
    }
}

fn pad_header(value: Vec<u8>, length: usize) -> Vec<u8> {
    let vec = Vec::with_capacity(length);
    value.truncate(length)
}
