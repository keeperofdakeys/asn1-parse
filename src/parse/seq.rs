use data::{Asn1Seq, Asn1SeqField};
use parse::asn1_field;
use parse::space::skip_other;

named!(pub asn1_seq_field <Asn1SeqField>, call!(asn1_field));

named!(pub asn1_seq <Asn1Seq>, do_parse!(
  tag!("SEQUENCE") >>
  opt!(skip_other) >>
  fields: delimited!(
    tag!("{"),
    separated_list!(
      do_parse!(opt!(skip_other) >> tag!(",") >> ()),
      asn1_seq_field
    ),
    tuple!(opt!(skip_other), tag!("}"))
  ) >>
  (Asn1Seq {
    fields: fields,
  })
));

#[test]
fn test_sequence() {
  let seq = Asn1Seq {
    fields: vec![
      Asn1SeqField {
        name: "foo".into(),
        tag: None,
        asn1_type: ::Asn1Type::Type("Bar".into()),
        optional: None,
      },
      Asn1SeqField {
        name: "asdf".into(),
        tag: None,
        asn1_type: ::Asn1Type::Integer(::Asn1Integer),
        optional: None,
      }
    ],
  };
  assert_eq!(
    seq,
    asn1_seq("\
      SEQUENCE { \
        foo Bar, \
        asdf INTEGER \
      } \
    ".as_bytes()).unwrap().1
  );
  assert_eq!(
    seq,
    asn1_seq("\
      SEQUENCE {
        foo Bar --,
        , asdf INTEGER
      }
    ".as_bytes()).unwrap().1
  );
  assert!(asn1_seq("SEQUENC ".as_bytes()).is_err());
}
