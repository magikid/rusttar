/*
    Copyright (C) 2017 Chris Jones

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

mod archive;
use archive::Archive;

extern crate getopts;
use getopts::Options;
use std::env;

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
}
