use HeaderField;

#[derive(Debug)]
pub struct NameField {
    value: String
}

impl HeaderField for NameField{
    fn as_bytes(&self) -> Vec<u8> {
        if (self.value.len()) > self.len() {
            self.truncate()
        } else {
            self.pad()
        }
    }

    fn len(&self) -> usize {
        100
    }

    fn truncate(&self) -> Vec<u8>{
        let num_cars_to_remove = self.value.len() - self.len() / 8;
        let truncate_val = self.value.len() - num_cars_to_remove;
        let mut new_field = self.as_bytes().to_vec();
        new_field.truncate(truncate_val);
        new_field
    }

    fn pad(&self) -> Vec<u8> {
        let mut val = self.value.clone().as_bytes().to_vec();
        let pad_length = match self.len().checked_sub(self.value.len()) {
            Some(n) => n,
            None => panic!("Problem with the name field length")
        };

        for _ in 1..pad_length+1 {
            val.push(0);
        }

        val
    }
}

#[cfg(test)]
mod tests{
    use name_field::NameField;
    use HeaderField;

    #[test]
    fn it_pads_short_values(){
        let n = NameField{ value: "foobar".to_string() };
        assert_eq!(n.as_bytes().len(), 100);
    }

    #[test]
    fn it_truncates_long_values(){
        let n = NameField{ value: "foobarfoobarbaz".to_string() };
        assert_eq!(n.as_bytes().len(), 100);
    }
}
