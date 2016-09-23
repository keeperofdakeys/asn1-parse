#[macro_use]
extern crate nom;

use nom::{space,multispace,is_alphanumeric,eol};

// LDAPString ::= OCTET STRING -- UTF-8 encoded,
//                             -- [ISO10646] characters

#[derive(Debug)]
enum Asn1Type {
  Type(String),
  Seq(Asn1Seq),
}

#[derive(Debug)]
struct Asn1Def {
  name: String,
  assign: Asn1Type,
}

#[derive(Debug)]
struct Asn1Seq {
  fields: Vec<Asn1Def>,
}

named!(type_name <&[u8], String>, chain!(
  s: take_while!(nom::is_alphanumeric),
  || String::from_utf8(Vec::from(s)).unwrap()
));

named!(type_assignment <&[u8], Asn1Def>, chain!(
  skip_other? ~
  name: type_name ~
  skip_other ~
  tag!("::=") ~
  skip_other ~
  assign: type_name,
  || Asn1Def {
    name: name,
    assign: Asn1Type::Type(assign),
  }
));

named!(type_sequence <&[u8], Asn1Seq>, chain!(
  tag!("SEQUNECE"),
  || Asn1Seq {
    fields: Vec::new(),
  }
));

pub fn is_eol(byte: &u8) -> bool {
  let chr = *byte as char;
  println!("{}", byte);
  chr == '\n' || chr == '\r'
}

named!(comment <()>, chain!(
  complete!(tag!("--")) ~
  take_till!(is_eol) ~
  eol,
  || ()
));

named!(skip_other <()>, chain!(
  multispace? ~
  comment? ~
  multispace? ~
  chain!(
     complete!(peek!(tag!("--"))) ~
     complete!(skip_other),
     || ()
  )?,
  || ()
));

named!(test_ten, chain!(
  skip_other ~
  s: take_while!(nom::is_alphanumeric),
  || s
));

fn main() {
  println!("{:#?}", type_assignment("test -- ::=\n      	-- :\n::= hi".as_bytes()));
  println!("{:#?}", String::from_utf8(test_ten(" --fds\n --\ndlsjfs\nfds ::= hi".as_bytes()).unwrap().0.to_owned()).unwrap());
}
