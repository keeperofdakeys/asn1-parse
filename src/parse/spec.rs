use data::{Asn1Spec, Tagging};
use parse::asn1_type_def;
use parse::space::skip_other;

use nom::{eol, space, IResult};

enum SpecErr {
  Custom(u32)
}

impl From<u32> for SpecErr {
  fn from(err: u32) -> SpecErr {
    SpecErr::Custom(err)
  }
}

macro_rules! conv_nom_error {
  ($submac:ident!( $($args:tt)* )) => (
    {
      match $submac!($($args)*) {
        IResult::Incomplete(x) => IResult::Incomplete(x),
        IResult::Done(i, o)    => IResult::Done(i, o),
        IResult::Error(err) => {
          match err {
            ErrorKind::Custom(e) => SpecErr::Custom(e),
            _ => err,
          }
        }
      }
    }
  );
}

named!(pub asn1_spec <&[u8], Asn1Spec, SpecErr>, conv_nom_error!( do_parse!(
  // TODO: Parse asn1 definition name (ldap has numbers after it?)
  do_parse!(
    take_until_and_consume!("DEFINITIONS") >>
    opt!(skip_other) >>
    ()
  ) >>
  // TODO: Handle explicit tags, extensibility not implied?, and maybe other things
  do_parse!(
    tag!("IMPLICIT TAGS") >>
    opt!(skip_other) >>
    ()
  ) >>
  opt!(do_parse!(
    tag!("EXTENSIBILITY IMPLIED") >>
    opt!(skip_other) >>
    ()
  )) >>
  tag!("::=") >>
  opt!(skip_other) >>
  do_parse!(
    take_until_and_consume!("BEGIN") >>
    opt!(space) >>
    eol >>
    ()
  ) >>
  a: opt!(skip_other) >>
  defs: many_till!(
    do_parse!(
      def: asn1_type_def >>
      opt!(skip_other) >>
      (def)
    ),
    tag!("END")
  ) >>
  (Asn1Spec {
    tagging: Tagging::Implicit,
    defs: defs.0
    // defs: vec![]
  })
)));

#[test]
fn test_asn1_spec() {
  assert_eq!(
    asn1_spec("
      DEFINITIONS
      IMPLICIT TAGS
      EXTENSIBILITY IMPLIED
      ::=
      BEGIN
      Foo ::= INTEGER
      Bar ::= [3] Foo
      Asdf ::= SET {
        qwerty Foo
      }
      END\
    ".as_bytes()).unwrap().1,
    ::Asn1Spec {
      tagging: ::Tagging::Implicit,
      defs: vec![
        ::Asn1Def {
          name: "Foo".into(),
          tag: None,
          assign: ::Asn1Type::Integer,
        },
        ::Asn1Def {
          name: "Bar".into(),
          tag: Some((::Asn1Class::ContextSpecific, 3)),
          assign: ::Asn1Type::Type("Foo".into()),
        },
        ::Asn1Def {
          name: "Asdf".into(),
          tag: None,
          assign: ::Asn1Type::Set(
            vec![::Asn1Field::Def(::Asn1FieldDef {
              name: "qwerty".into(),
              tag: None,
              asn1_type: ::Asn1Type::Type("Foo".into()),
              optional: None,
            })]
          ),
        }
      ],
    }
  );
}
