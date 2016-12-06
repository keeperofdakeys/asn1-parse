use data::{Asn1Set, Asn1SetField};
use parse::asn1_field;
use parse::space::{skip_other};

named!(pub asn1_set_field <Asn1SetField>, call!(asn1_field));

named!(pub asn1_set <Asn1Set>, chain!(
  tag!("SET") ~
  skip_other? ~
  fields: delimited!(
    tag!("{"),
    separated_list!(
      chain!(skip_other? ~ tag!(","), || ()),
      asn1_set_field
    ),
    tuple!(opt!(skip_other), tag!("}"))
  ),
  || Asn1Set {
    fields: fields,
  }
));

#[test]
fn test_set() {
  let set = Asn1Set {
    fields: vec![
      Asn1SetField {
        name: "foo".into(),
        tag: None,
        asn1_type: ::Asn1Type::Type("Bar".into()),
        optional: false,
      },
      Asn1SetField {
        name: "asdf".into(),
        tag: None,
        asn1_type: ::Asn1Type::Integer(::Asn1Integer),
        optional: false,
      }
    ],
  };
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
