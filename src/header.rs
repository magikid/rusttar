use std::fmt;

struct HeaderField {
    value: Vec<u8>,
    length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref val = self.value;
        let length = self.length;
        let pad_length = length - (val.len() * 8);

        for byte in val.iter(){
            try!(write!(f, "{:08b}", byte));
        }

        for _ in 1..pad_length {
            try!(write!(f, "0"));
        }

        write!(f, "{}", "\0")
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
