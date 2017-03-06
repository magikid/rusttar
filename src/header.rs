use std::fmt;

struct HeaderField {
    value: Vec<u8>,
    length: usize
}

impl fmt::Display for HeaderField{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes_to_write = pad_field(self.value.clone(), self.length);

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

fn pad_field(value: Vec<u8>, length: usize) -> Vec<u8> {
    let mut new_vec: Vec<u8> = Vec::with_capacity(length);
    for (n, _) in  new_vec.clone().iter_mut().enumerate() {
        match value.get(n) {
            Some(val) => new_vec.insert(n, val.clone()),
            None => new_vec.insert(n, 0),
        }
    }
    new_vec
}
