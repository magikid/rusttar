use std::fs::File;
use std::io::Read;

pub struct Archive {
    pub file: String,
}

impl Archive {
    pub fn listing(&self) -> Vec<String>{
        let mut file_contents = Vec::new();
        match File::open(self.file.clone()) {
            Ok(mut f) => f.read_to_end(&mut file_contents),
            Err(f) => panic!{ f.to_string() }
        };
        println!("{:?}", file_contents);
        vec![self.file.clone()]
    }
}
