extern crate clap;
extern crate libc;
extern crate nix;

use clap::{App, Arg};
use libc::{STDIN_FILENO, STDOUT_FILENO};
use nix::fcntl::{open, OFlag};
use nix::unistd::{read, write, close};
use nix::sys::stat::Mode;
use std::path::Path;
use std::env;
use std::usize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = App::new(args[0].clone())
                      .version("0.1")
                      .arg(Arg::with_name("pathname")
                           .required(true))
                      .arg(Arg::with_name("append")
                           .short("a")
                           .long("append")
                           .help("append to file instead of truncating"))
                      .get_matches();


    let pathname = Path::new(matches.value_of("pathname").unwrap());
    // prepare flags
    let mut flags = OFlag::O_CREAT | OFlag::O_WRONLY;
    // test -a argument, return value of match ored in flags
    flags |= match matches.is_present("append") {
        true => OFlag::O_APPEND,
        false => OFlag::O_TRUNC,
    };
    let mode = Mode::S_IRUSR | Mode::S_IWUSR;
    let fd = open(pathname, flags, mode).unwrap();

    // init buf to 0
    let mut buf: [u8; 4096] = [0; 4096];
    let mut nb_read: usize = 1;
    while nb_read > 0 {
        // read stdin
        nb_read = read(STDIN_FILENO, &mut buf).unwrap();
        // write to output file
        write(fd, &buf[..nb_read]).unwrap();
        // write to stdout
        write(STDOUT_FILENO, &buf[..nb_read]).unwrap();
    }

    // close
    close(fd).unwrap();
}
