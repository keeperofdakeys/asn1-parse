use data::Asn1Type;
use parse::asn1_field;
use parse::space::skip_other;

named!(pub asn1_set <Asn1Type>, do_parse!(
  tag!("SET") >>
  opt!(skip_other) >>
  fields: delimited!(
    tag!("{"),
    separated_list!(
      do_parse!(opt!(skip_other) >> tag!(",") >> ()),
      do_parse!(opt!(skip_other) >> f: asn1_field >> (f))
    ),
    tuple!(opt!(skip_other), tag!("}"))
  ) >>
  (Asn1Type::Set(fields))
));

#[test]
fn test_set() {
  let set = Asn1Type::Set(
    vec![
      ::Asn1Field::Def(::Asn1FieldDef {
        name: "foo".into(),
        tag: None,
        asn1_type: ::Asn1Type::Type("Bar".into()),
        optional: None,
      }),
      ::Asn1Field::Def(::Asn1FieldDef {
        name: "asdf".into(),
        tag: None,
        asn1_type: ::Asn1Type::Integer,
        optional: None,
      })
    ]
  );
  assert_eq!(
    set,
    asn1_set("\
      SET {\
        foo Bar,\
        asdf INTEGER\
      }\
    ".as_bytes()).unwrap().1
  );
  assert_eq!(
    set,
    asn1_set("\
      SET {
        foo Bar --,
        , asdf INTEGER
      }
    ".as_bytes()).unwrap().1
  );
  assert!(asn1_set("SE ".as_bytes()).is_err());
}
