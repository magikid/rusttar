extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Read;

struct Archive {
    file: String,
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "", "the tar archive to read", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("t", "list", "list the archive contents");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") || !matches.opt_present("f"){
        print_usage(&program, opts);
        return;
    }

    let archive = matches.opt_str("f");
    if matches.opt_present("t") {
        list(archive);
    } else {
        println!("You didn't specify an operation!");
        print_usage(&program, opts);
    }
}

fn list(file_argument: Option<String>){
    let filename = match file_argument{
        Some(n) => { n },
        None => { panic!("Filename problem") }
    };
    let archive = Archive { file: filename };
    println!("{}",archive.listing().join("\n"));
    println!("Work, work, work!");
}
