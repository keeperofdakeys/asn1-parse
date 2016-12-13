pub mod space;
pub mod seq;
pub mod set;
pub mod choice;
pub mod int;
pub mod spec;

use nom::{is_alphanumeric, alpha, digit};
use parse::space::skip_other;
use parse::seq::asn1_seq;
use parse::set::asn1_set;
use parse::choice::asn1_choice;
use parse::int::asn1_integer;
use data::{Asn1Type, Asn1Class, Asn1Tag, Asn1Def, Asn1Field, Asn1Optional};

named!(pub asn1_class_tag <Asn1Tag>, do_parse!(
  class: opt!(alpha) >>
  opt!(skip_other) >>
  tag_num: digit >>
  ( {
      let class = class.and_then(|i|
        String::from_utf8(Vec::from(i)).ok()
      );
      Asn1Class::parse(class).expect("Couldn't parse tag class")
    },
    String::from_utf8(Vec::from(tag_num)).unwrap().parse().expect("Couldn't parse tag number")
  )
));

named!(pub asn1_tag <Asn1Tag>, do_parse!(
  tag: delimited!(
    tag!("["),
    asn1_class_tag,
    tuple!(opt!(skip_other), tag!("]"))
  ) >>
  (tag)
));

named!(pub asn1_type_def <Asn1Def>, do_parse!(
  name: asn1_type_name >>
  opt!(skip_other) >>
  tag!("::=") >>
  opt!(skip_other) >>
  tag: opt!(asn1_tag) >>
  opt!(skip_other) >>
  asn1_type: asn1_type >>
  (Asn1Def {
    name: name,
    tag: tag,
    assign: asn1_type,
  })
));

named!(pub asn1_type <Asn1Type>, alt!(
  asn1_seq |
  asn1_set |
  asn1_choice |
  asn1_integer |
  asn1_assignment
));

named!(pub asn1_type_name <String>, do_parse!(
  s: alt!(
    tag!("BIT STRING") |
    tag!("OCTET STRING") |
    tag!("OBJECT IDENTIFIER") |
    tag!("INSTANCE OF") |
    tag!("EMBEDDED PDV") |
    tag!("CHARACTER STRING") |
    take_while!(is_alphanumeric)
  ) >>
  (String::from_utf8(Vec::from(s)).unwrap())
));

named!(pub asn1_assignment <Asn1Type>, do_parse!(
  t: asn1_type_name >>
  (Asn1Type::Type(t))
));

named!(pub asn1_field <Asn1Field>, do_parse!(
  name: asn1_type_name >>
  opt!(skip_other) >>
  tag: opt!(asn1_tag) >>
  opt!(skip_other) >>
  asn1_type: asn1_type >>
  opt!(skip_other) >>
  optional: opt!(alt!(asn1_field_optional | asn1_field_default)) >>
  (Asn1Field {
    name: name,
    tag: tag,
    asn1_type: asn1_type,
    optional: optional,
  })
));

named!(pub asn1_field_optional <Asn1Optional>, do_parse!(
  tag!("OPTIONAL") >>
  (Asn1Optional::Optional)
));

named!(pub asn1_field_default <Asn1Optional>, do_parse!(
  tag!("DEFAULT") >>
  opt!(skip_other) >>
  default: take_while!(is_alphanumeric) >>
  (Asn1Optional::Default(
    String::from_utf8(Vec::from(default)).unwrap()
  ))
));

#[test]
fn test_asn1_type() {
  assert_eq!(asn1_type_name("Foo Bar".as_bytes()).unwrap().1, "Foo");
  assert_eq!(asn1_type_name("OCTET STRING".as_bytes()).unwrap().1, "OCTET STRING");
}

#[test]
fn test_asn1_tag() {
  let tag: Asn1Tag = (Asn1Class::ContextSpecific, 32);
  assert_eq!(asn1_tag("[32]".as_bytes()).unwrap().1, tag);
  assert_eq!(asn1_tag("[ 32 ]".as_bytes()).unwrap().1, tag);
  assert_eq!(
    asn1_tag("[APPLICATION 12]".as_bytes()).unwrap().1,
    (Asn1Class::Application, 12)
  );
  assert_eq!(
    asn1_tag("[PRIVATE 24]".as_bytes()).unwrap().1,
    (Asn1Class::Private, 24)
  );
}

#[test]
fn test_asn1_field() {
  let field1 = ::Asn1Field {
    name: "foo".into(),
    tag: None,
    asn1_type: ::Asn1Type::Type("Bar".into()),
    optional: None,
  };
  let field2 = ::Asn1Field {
    name: "asdf".into(),
    tag: Some((Asn1Class::Application, 9)),
    asn1_type: ::Asn1Type::Integer,
    optional: Some(Asn1Optional::Optional),
  };
  let field3 = ::Asn1Field {
    name: "sample".into(),
    tag: None,
    asn1_type: ::Asn1Type::Integer,
    optional: Some(Asn1Optional::Default("TRUE".into())),
  };
  assert_eq!(
    field1,
    asn1_field("foo Bar,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field2,
    asn1_field("asdf [APPLICATION 9] INTEGER OPTIONAL,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field1,
    asn1_field("foo--test\n Bar,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field3,
    asn1_field("sample INTEGER DEFAULT TRUE,".as_bytes()).unwrap().1
  );
}
