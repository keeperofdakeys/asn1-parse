use data::{Asn1Choice, Asn1ChoiceField};
use parse::asn1_field;
use parse::space::{skip_other};

named!(pub asn1_choice_field <Asn1ChoiceField>, call!(asn1_field));

named!(pub asn1_choice <Asn1Choice>, chain!(
  tag!("CHOICE") ~
  skip_other? ~
  fields: delimited!(
    tag!("{"),
    separated_list!(
      chain!(skip_other? ~ tag!(","), || ()),
      asn1_choice_field
    ),
    tuple!(opt!(skip_other), tag!("}"))
  ),
  || Asn1Choice {
    fields: fields,
  }
));

#[test]
fn test_choice() {
  let choice = Asn1Choice {
    fields: vec![
      Asn1ChoiceField {
        name: "foo".into(),
        tag: None,
        asn1_type: ::Asn1Type::Type("Bar".into()),
      },
      Asn1ChoiceField {
        name: "asdf".into(),
        tag: None,
        asn1_type: ::Asn1Type::Integer(::Asn1Integer),
      }
    ],
  };
  assert_eq!(
    choice,
    asn1_choice("\
      CHOICE {\
        foo Bar,\
        asdf INTEGER\
      }\
    ".as_bytes()).unwrap().1
  );
  assert_eq!(
    choice,
    asn1_choice("\
      CHOICE {
        foo Bar --,
        , asdf INTEGER
      }
    ".as_bytes()).unwrap().1
  );
  assert!(asn1_choice("CHOIC ".as_bytes()).is_err());
}
