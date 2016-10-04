#[derive(PartialEq, Debug)]
pub enum Asn1Type {
  Type(String),
  Seq(Asn1Seq),
}

#[derive(PartialEq, Debug)]
pub struct Asn1Def {
  pub name: String,
  pub assign: Asn1Type,
}

#[derive(PartialEq, Debug)]
pub struct Asn1Seq {
  pub fields: Vec<Asn1SeqField>,
}

pub type Asn1SeqField = Asn1Field;
#[derive(PartialEq, Debug)]
pub struct Asn1Field {
  pub name: String,
  pub asn1_type: Asn1Type,
}

