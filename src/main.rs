use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
use structopt::StructOpt;
use std::num::ParseIntError;

fn parse_hex(s: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(s, 16)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cut-trailing-bytes", about = "A tool for cut trailing bytes, default cut trailing NULL bytes(0x00 in hex)")]
struct Opt {
    /// File to cut
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// For example, pass 'ff' if want to cut 0xff
    #[structopt(short = "c", long = "cut-byte", default_value="0", parse(try_from_str = parse_hex))]
    byte_in_hex: u8,

    /// Check the file but don't real cut it
    #[structopt(short, long = "dry-run")]
    dry_run: bool,
}


fn main() -> io::Result<()> {

    let opt = Opt::from_args();
    let filename = &opt.file;
    let mut f = File::open(filename)?;
    let mut valid_len = 0;
    let mut tmp_len = 0;
    let mut buffer = [0; 4096];

    loop {
        let mut n = f.read(&mut buffer[..])?;
        if n == 0 { break; }
        for byte in buffer.bytes() {
            match byte.unwrap() {
                byte if byte == opt.byte_in_hex => { tmp_len += 1; }
                _ => {
                    valid_len += tmp_len;
                    tmp_len = 0;
                    valid_len += 1;
                }
            }
            n -= 1;
            if n == 0 { break; }
        }
    }
    if !opt.dry_run {
        let f = OpenOptions::new().write(true).open(filename);
        f.unwrap().set_len(valid_len)?;
    }
    println!("cut {} from {} to {}", filename.display(), valid_len + tmp_len, valid_len);

    Ok(())
}

