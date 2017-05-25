use nom::is_alphanumeric;
use nom::space;

use data::Asn1Type;
use parse::asn1_type;
use parse::space::skip_other;

named!(pub asn1_seq_of <Asn1Type>, do_parse!(
  tag!("SEQUENCE OF") >>
  opt!(skip_other) >>
  t: alt!(
    do_parse!(
      s: take_while1!(is_alphanumeric) >>
      // FIXME: We should use skip_other here, but the alt!
      // macro call requires a more strict termination point.
      // (The second option fails otherwise).
      space >>
      t: asn1_type >>
      (Some(s), t)
    ) |
    do_parse!(
      t: asn1_type >>
      (None, t)
    )
  ) >>
  (Asn1Type::SeqOf(
    t.0.and_then(|s| Some(String::from_utf8(Vec::from(s)).unwrap())),
    Box::new(t.1)
  ))
));

#[test]
fn test_seq_of() {
  assert_eq!(
    Asn1Type::SeqOf(None, Box::new(Asn1Type::Integer)),
    asn1_seq_of("SEQUENCE OF INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    Asn1Type::SeqOf(Some("value".into()), Box::new(Asn1Type::Type("OCTET STRING".into()))),
    asn1_seq_of("SEQUENCE OF value OCTET STRING,".as_bytes()).unwrap().1
  );
  assert!(asn1_seq_of("SEQUENCE O ".as_bytes()).is_err());
}
