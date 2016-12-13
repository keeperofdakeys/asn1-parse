use data::Asn1Type;
use parse::asn1_type;
use parse::space::skip_other;

named!(pub asn1_set_of <Asn1Type>, do_parse!(
  tag!("SET OF") >>
  opt!(skip_other) >>
  t: asn1_type >>
  (Asn1Type::SetOf(Box::new(t)))
));

#[test]
fn test_set_of() {
  assert_eq!(
    Asn1Type::SetOf(Box::new(Asn1Type::Integer)),
    asn1_set_of("SET OF INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    Asn1Type::SetOf(Box::new(Asn1Type::Type("OCTET STRING".into()))),
    asn1_set_of("SET OF  OCTET STRING,".as_bytes()).unwrap().1
  );
  assert!(asn1_set_of("SET O ".as_bytes()).is_err());
}
