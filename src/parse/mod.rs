pub mod space;
pub mod seq;
pub mod choice;

use nom::{space,is_alphanumeric};
use parse::space::{skip_other};
use parse::seq::asn1_seq;
use parse::choice::asn1_choice;
use data::{Asn1Type, Asn1Def, Asn1Field};

named!(pub asn1_type_name <String>, chain!(
  s: take_while!(is_alphanumeric),
  || String::from_utf8(Vec::from(s)).unwrap()
));

named!(pub asn1_type_def <Asn1Def>, chain!(
  skip_other? ~
  name: asn1_type_name ~
  space? ~
  tag!("::=") ~
  space? ~
  asn1_type: asn1_type,
  || Asn1Def {
    name: name,
    assign: asn1_type,
  }
));

named!(pub asn1_type <Asn1Type>, alt!(
  chain!(s: asn1_seq, || Asn1Type::Seq(s)) |
  chain!(c: asn1_choice, || Asn1Type::Choice(c)) |
  chain!(t: asn1_assignment, || Asn1Type::Type(t))
));

named!(pub asn1_assignment <String>, chain!(
  t: asn1_type_name,
  || t
));

named!(pub asn1_field <Asn1Field>, chain!(
  skip_other? ~
  name: asn1_type_name ~
  skip_other? ~
  asn1_type: asn1_type,
  || Asn1Field {
    name: name,
    asn1_type: asn1_type,
  }
));

#[test]
fn test_asn1_field() {
  let field1 = ::Asn1Field {
    name: "foo".into(),
    asn1_type: ::Asn1Type::Type("Bar".into()),
  };
  let field2 = ::Asn1Field {
    name: "asdf".into(),
    asn1_type: ::Asn1Type::Type("INTEGER".into()),
  };
  assert_eq!(
    field1,
    asn1_field("foo Bar".as_bytes()).unwrap().1
  );
  assert_eq!(
    field2,
    asn1_field("asdf INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    field1,
    asn1_field("foo--test\n Bar".as_bytes()).unwrap().1
  );
}
