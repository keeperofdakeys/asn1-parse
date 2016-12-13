use data::Asn1Spec;
use parse::asn1_type_def;
use parse::space::skip_other;

named!(pub asn1_spec <Asn1Spec>, do_parse!(
  tag!("BEGIN") >>
  opt!(skip_other) >>
  defs: many_till!(
    do_parse!(
      def: asn1_type_def >>
      opt!(skip_other) >>
      (def)
    ),
    tag!("END")
  ) >>
  (Asn1Spec {
    defs: defs.0
  })
));

#[test]
fn test_asn1_spec() {
  assert_eq!(
    asn1_spec("BEGIN
      Foo ::= INTEGER
      Bar ::= [3] Foo
      Asdf ::= SET {
        qwerty Foo
      }
      END\
    ".as_bytes()).unwrap().1,
    ::Asn1Spec {
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
            vec![::Asn1Field {
              name: "qwerty".into(),
              tag: None,
              asn1_type: ::Asn1Type::Type("Foo".into()),
              optional: None,
            }]
          ),
        }
      ],
    }
  );
}
