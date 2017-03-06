use std::fmt;

struct HeaderField {
    value: Vec<u8>,
    length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut val = self.value.clone();
        let length = self.length;
        let value_length = val.len() * 8;
        if length > value_length {
            let pad_length = length - value_length;

            for byte in val.iter(){
                try!(write!(f, "{:08b}", byte));
            }

            for _ in 1..pad_length {
                try!(write!(f, "0"));
            }

        } else {
            println!("Truncated {} bytes", value_length - length);
            for byte in val.iter(){
                try!(write!(f, "{:08b}", byte));
            }
        }

        write!(f, "{}", "\0")
    }
}

#[cfg(test)]
mod tests{
    use header::HeaderField;

    #[test]
    fn it_pads_short_values(){
        let foobar_field = "foobar".to_string().as_bytes().to_vec();
        let h = HeaderField { value: foobar_field, length: 100 };
        let output = format!("{}", h);
        assert_eq!(output.len(), h.length);
    }

    #[test]
    fn it_truncates_long_values(){
        let foobar_field = "foobarfoobarbaz".to_string().as_bytes().to_vec();
        let h = HeaderField { value: foobar_field, length: 100 };
        let output = format!("{}", h);
        assert_eq!(output.len(), h.length);
    }
}
