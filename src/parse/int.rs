use data::Asn1Integer;
use parse::space::skip_other;

named!(pub asn1_integer <Asn1Integer>, do_parse!(
  tag!("INTEGER") >>
  opt!(skip_other) >>
  // TODO: Don't ignore all this stuff
  opt!(do_parse!(
    tag!("{") >>
    take_until!("}") >>
    tag!("}") >>
    ()
  )) >>
  (Asn1Integer {})
));
