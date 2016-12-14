use nom::is_alphanumeric;

use data::Asn1Type;
use parse::asn1_type;
use parse::space::skip_other;

named!(pub asn1_set_of <Asn1Type>, do_parse!(
  tag!("SET OF") >>
  opt!(skip_other) >>
  t: alt!(
    do_parse!(
      s: take_while1!(is_alphanumeric) >>
      skip_other >>
      t: asn1_type >>
      (Some(s), t)
    ) |
    do_parse!(
      t: asn1_type >>
      (None, t)
    )
  ) >>
  (Asn1Type::SetOf(
    t.0.and_then(|s| Some(String::from_utf8(Vec::from(s)).unwrap())),
    Box::new(t.1)
  ))
));

#[test]
fn test_set_of() {
  assert_eq!(
    Asn1Type::SetOf(None, Box::new(Asn1Type::Integer)),
    asn1_set_of("SET OF INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    Asn1Type::SetOf(Some("value".into()), Box::new(Asn1Type::Type("OCTET STRING".into()))),
    asn1_set_of("SET OF value OCTET STRING,".as_bytes()).unwrap().1
  );
  assert!(asn1_set_of("SET O ".as_bytes()).is_err());
}
