extern crate asn1_parse;
#[macro_use]
extern crate nom;
extern crate argparse;

use std::io;
use std::io::Read;
use std::fs;
use std::path::Path;
use asn1_parse::parse::spec::asn1_spec;
use argparse::{ArgumentParser, StoreOption};

fn main() {
  let opts = parse_args();

  let bytes: Result<Vec<_>, _> = match opts.file {
    Some(ref p) => io::BufReader::new(
      fs::File::open(Path::new(p)).unwrap()
    ).bytes().collect(),
    None => io::stdin().bytes().collect(),
  };
  let buffer = bytes.unwrap();
  let elems: nom::IResult<_, _> = many0!(buffer.as_slice(), complete!(asn1_spec));
  println!("{:#?}", elems);
}

struct ProgOpts {
  file: Option<String>,
}

fn parse_args() -> ProgOpts {
  let mut opts = ProgOpts {
    file: None,
  };

  {
    let mut ap = ArgumentParser::new();
    ap.set_description("Dump ASN.1 specification files");
    ap.refer(&mut opts.file)
      .add_argument("file", StoreOption, "ASN.1 spec file to dump");
    ap.parse_args_or_exit();
  }
  opts
}
