pub mod space;
pub mod seq;
pub mod set;
pub mod choice;
pub mod int;
pub mod spec;

use nom::{space, is_alphanumeric, alpha, digit};
use parse::space::{skip_other};
use parse::seq::asn1_seq;
use parse::set::asn1_set;
use parse::choice::asn1_choice;
use parse::int::asn1_integer;
use data::{Asn1Type, Asn1Class, Asn1Tag, Asn1Def, Asn1Field};

named!(pub asn1_type_name <String>, chain!(
  s: take_while!(is_alphanumeric),
  || String::from_utf8(Vec::from(s)).unwrap()
));

named!(pub asn1_class_tag <Asn1Tag>, do_parse!(
  opt!(space) >>
  class: opt!(alpha) >>
  opt!(space) >>
  tag_num: digit >>
  opt!(space) >>
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
  opt!(space) >>
  tag: delimited!(
    tag!("["),
    asn1_class_tag,
    tag!("]")
  ) >>
  (tag)
));

named!(pub asn1_type_def <Asn1Def>, do_parse!(
  name: asn1_type_name >>
  opt!(space) >>
  tag!("::=") >>
  tag: opt!(asn1_tag) >>
  opt!(space) >>
  asn1_type: asn1_type >>
  (Asn1Def {
    name: name,
    tag: tag,
    assign: asn1_type,
  })
));

named!(pub asn1_type <Asn1Type>, alt!(
  chain!(s: asn1_seq, || Asn1Type::Seq(s)) |
  chain!(s: asn1_set, || Asn1Type::Set(s)) |
  chain!(c: asn1_choice, || Asn1Type::Choice(c)) |
  chain!(i: asn1_integer, || Asn1Type::Integer(i)) |
  chain!(t: asn1_assignment, || Asn1Type::Type(t))
));

named!(pub asn1_assignment <String>, chain!(
  t: asn1_type_name,
  || t
));

named!(pub asn1_field <Asn1Field>, do_parse!(
  opt!(skip_other) >>
  name: asn1_type_name >>
  opt!(skip_other) >>
  tag: opt!(asn1_tag) >>
  opt!(skip_other) >>
  asn1_type: asn1_type >>
  opt!(skip_other) >>
  optional: opt!(tag!("OPTIONAL")) >>
  (Asn1Field {
    name: name,
    tag: tag,
    asn1_type: asn1_type,
    optional: optional.is_some(),
  })
));

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
    optional: false,
  };
  let field2 = ::Asn1Field {
    name: "asdf".into(),
    tag: Some((Asn1Class::Application, 9)),
    asn1_type: ::Asn1Type::Integer(::Asn1Integer),
    optional: false,
  };
  let field3 = ::Asn1Field {
    name: "sample".into(),
    tag: None,
    asn1_type: ::Asn1Type::Integer(::Asn1Integer),
    optional: true,
  };
  assert_eq!(
    field1,
    asn1_field("foo Bar,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field2,
    asn1_field("asdf [APPLICATION 9] INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field1,
    asn1_field("foo--test\n Bar,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field3,
    asn1_field("sample INTEGER OPTIONAL,".as_bytes()).unwrap().1
  );
}
