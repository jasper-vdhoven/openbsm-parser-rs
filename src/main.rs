use clap::{Parser, ValueEnum};
use std::io::{Read, BufReader, self, Cursor};
use std::fs::File;
use std::path::{Path, PathBuf};
use binread::{BinReaderExt, BinRead};
//use bytes::{Buf, BigEndian};
use byteorder::{BigEndian, ReadBytesExt};

// OpenBSM Structs
/*
 * record byte count       4 bytes
 * version #               1 byte    [2]
 * event type              2 bytes
 * event modifier          2 bytes
 * seconds of time         4 bytes/8 bytes (32-bit/64-bit value)
 * milliseconds of time    4 bytes/8 bytes (32-bit/64-bit value)
 */
#[derive(BinRead)]
#[br(big)]
struct UHeader32T {
    size:       u32,
    version:    char,
    e_type:     u16,
    e_mod:      u16,
    s:          u32,
    ms:         u32
}


/// arg parser
#[derive(Parser, Debug)]
struct Args {
    // Input
    #[arg(short, long, required = false, value_name = "./logs/openbsmlog")]
    input: Option<PathBuf>,
    // Output
    #[arg(short, long, required = false, value_name = "./output/parsed-log.xml")]
    output: Option<PathBuf>,
    // passwd file path
    #[arg(short, long, required = false, value_name = "/etc/passwd")]
    passwd: Option<PathBuf>,
    // groups file path
    #[arg(short, long, required = false ,value_name = "/etc/groups")]
    groups: Option<PathBuf>,
    // log level setting
    #[arg(value_enum, short('l'), long, required = false ,default_value_t = LogLevels::Error)]
    loglevel: LogLevels,
    // log file
    #[arg(short('f'), long, required = false, value_name = "./log-file.log")]
    logfile: Option<PathBuf>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevels {
    // verbose
    Debug,
    // informative
    Info,
    // Warnings but not worthy of errors
    Warning,
    // Default state; something is wrong that needs to be checked
    Error,
    // Can't continue
    Critical
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("OpenBSM Rust parser");

    println!("[i] Active Args:");
    println!("[-] Input file: {:?}", &args.input);
    println!("[-] Output file: {:?}", &args.output);
    println!("[-] Passwd file: {:?}", &args.passwd);
    println!("[-] Groups file: {:?}", &args.groups);
    println!("[-] Log level: {:?}", &args.loglevel);
    println!("[-] Log file: {:?}", &args.logfile);
    println!("[i] Starting parser now...\n\n");

    // Open input file for reading
    let mut fh = File::open(args.input.unwrap())?;
    let mut first_byte = [0;1];
    fh.read_exact(&mut first_byte)?;
    
    println!("First byte: 0x{:02x?}", first_byte[0]);

    match first_byte[0] {
        0x14 => fetch_header32(&mut fh),
        _ => println!("[!] Invalid byte red"),
    }


    Ok(())
}

fn fetch_header32(fh: &mut File) {
    println!("[i] Fetching header 32 struct");
    let mut buffer = vec![0,64];
    fh.read_exact(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    let header32_size = cursor.read_u32::<BigEndian>().unwrap();
    let header32_version = cursor.read_char::<BigEndian>().unwrap();
    let header32_etype = cursor.read_u16::<BigEndian>().unwrap();
    let header32_emod = cursor.read_u16::<BigEndian>().unwrap();
    let header32_s = cursor.read_u32::<BigEndian>().unwrap();
    let header32_ms = cursor.read_u32::<BigEndian>().unwrap();
}
