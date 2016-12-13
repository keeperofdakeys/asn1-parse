use data::{Asn1Type, Asn1ChoiceField};
use parse::asn1_field;
use parse::space::skip_other;

named!(pub asn1_choice_field <Asn1ChoiceField>, call!(asn1_field));

named!(pub asn1_choice <Asn1Type>, do_parse!(
  tag!("CHOICE") >>
  opt!(skip_other) >>
  fields: delimited!(
    tag!("{"),
    separated_list!(
      do_parse!(opt!(skip_other) >> tag!(",") >> ()),
      do_parse!(opt!(skip_other) >> f: asn1_choice_field >> (f))
    ),
    tuple!(opt!(skip_other), tag!("}"))
  ) >>
  (Asn1Type::Choice(fields))
));

#[test]
fn test_choice() {
  let choice = Asn1Type::Choice(
    vec![
      Asn1ChoiceField {
        name: "foo".into(),
        tag: None,
        asn1_type: ::Asn1Type::Type("Bar".into()),
        optional: None,
      },
      Asn1ChoiceField {
        name: "asdf".into(),
        tag: None,
        asn1_type: ::Asn1Type::Integer,
        optional: None,
      }
    ]
  );
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
