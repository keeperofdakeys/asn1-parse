use nom::multispace;

use data::Asn1Integer;

named!(pub asn1_integer <Asn1Integer>, chain!(
  tag!("INTEGER") ~
  multispace? ~
  // TODO: Don't ignore all this stuff
  chain!(
    tag!("{") ~
    take_until!("}") ~
    tag!("}"),
    || ()
  )? ,
  || Asn1Integer{}
));
