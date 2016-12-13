use data::Asn1Type;
use parse::asn1_type;
use parse::space::skip_other;

named!(pub asn1_seq_of <Asn1Type>, do_parse!(
  tag!("SEQUENCE OF") >>
  opt!(skip_other) >>
  t: asn1_type >>
  (Asn1Type::SeqOf(Box::new(t)))
));

#[test]
fn test_seq_of() {
  assert_eq!(
    Asn1Type::SeqOf(Box::new(Asn1Type::Integer)),
    asn1_seq_of("SEQUENCE OF INTEGER,".as_bytes()).unwrap().1
  );
  assert_eq!(
    Asn1Type::SeqOf(Box::new(Asn1Type::Type("OCTET STRING".into()))),
    asn1_seq_of("SEQUENCE OF  OCTET STRING,".as_bytes()).unwrap().1
  );
  assert!(asn1_seq_of("SEQUENCE O ".as_bytes()).is_err());
}
