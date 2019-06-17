extern crate libc;
extern crate nix;
use libc::{STDIN_FILENO, STDOUT_FILENO};
use nix::fcntl::{open, OFlag};
use nix::unistd::{read, write, close};
use nix::sys::stat::Mode;
use std::path::Path;
use std::env;
use std::usize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <pathname>", args[0]);
        return;
    }

    let pathname = Path::new(&args[1]);
    // open the file
    let flags = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
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
