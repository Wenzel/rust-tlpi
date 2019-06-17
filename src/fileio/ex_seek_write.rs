extern crate clap;
extern crate nix;

use clap::{App, Arg};
use nix::fcntl::{open, OFlag};
use nix::unistd::{write, lseek, close, Whence};
use nix::sys::stat::Mode;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = App::new(args[0].clone())
                      .version("0.1")
                      .arg(Arg::with_name("pathname")
                           .required(true))
                      .arg(Arg::with_name("offset")
                           .required(true))
                      .arg(Arg::with_name("string")
                           .required(true))
                      .get_matches();

    /* open pathname, seek to offset and write string */

    // parse args
    let pathname = Path::new(matches.value_of("pathname").unwrap());
    let offset = matches.value_of("offset").unwrap().parse::<i64>().unwrap();
    let string = matches.value_of("string").unwrap();

    let fd = open(pathname, OFlag::O_WRONLY, Mode::empty()).unwrap();

    lseek(fd, offset, Whence::SeekSet).unwrap();

    write(fd, string.as_bytes()).unwrap();

    close(fd).unwrap();
}
