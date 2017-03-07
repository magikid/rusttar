use std::fmt;

pub struct HeaderField {
    pub value: Vec<u8>,
    pub length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = truncate(self.value.clone(), self.length);
        let pad_length = match self.length.checked_sub(val.len() * 8) {
            Some(n) => n,
            None => panic!("Problem with the header length")
        };

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
    if (value.len() * 8) > length {
        // Header value is longer than allowed
        let num_chars_to_remove = value.len() - length / 8;
        let truncate_val = value.len() - num_chars_to_remove;
        let mut new_value = value;
        new_value.truncate(truncate_val);
        new_value
    } else {
        value
    }
}

impl HeaderField{
    pub fn as_bytes(&self) -> Box<[u8]> {
        let mut val = truncate(self.value.clone(), self.length);
        let pad_length = match self.length.checked_sub(val.len() * 8) {
            Some(n) => n,
            None => panic!("Problem with the header length")
        };

        for _ in 1..pad_length {
            val.push(0);
        }
        val.into_boxed_slice()
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
