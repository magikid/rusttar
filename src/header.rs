use std::fmt;

struct HeaderField {
    value: Vec<u8>,
    length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut val = self.value.clone();
        let length = self.length;
        let value_length = length * 8;
        let val = truncate(val, length);

        let pad_length = length - value_length;

        for byte in val.iter(){
            try!(write!(f, "{:08b}", byte));
        }

        for _ in 1..pad_length {
            try!(write!(f, "0"));
        }

        write!(f, "{}", "\0")
    }
}

fn truncate(value: Vec<u8>, length: usize) -> Vec<u8>{
    if (value.len() * 8) < length {
        value
    } else {
        let truncate_val = value.len() - (value.len() - length / 8);
        let mut new_value = value.clone();
        new_value.truncate(truncate_val);
        new_value
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
