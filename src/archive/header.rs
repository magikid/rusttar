use std::iter::FromIterator;

#[derive(Clone,Debug)]
pub struct Header{
    file_name: String,
}

#[derive(Clone,Debug)]
pub struct Headers{
    headers: Vec<Header>
}

impl Iterator for Headers {
    type Item = Header;
    fn next(&mut self) -> Option<Header> {
        self.headers.clone().into_iter().next()
    }
}

impl Headers {
    fn push(&mut self, elem: Header){
        self.headers.push(elem)
    }
}

impl FromIterator<u8> for Headers {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        let mut headers = Headers{ headers: Vec::new() };
        let mut file_name = Vec::new();

        for b in iter {
            file_name.push(b);
        }

        let header = Header { file_name: String::from_utf8(file_name).unwrap() };
        headers.push(header);

        headers
    }
}
