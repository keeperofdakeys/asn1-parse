#[derive(PartialEq, Debug)]
pub enum Asn1Type {
  Type(String),
  Seq(Asn1Seq),
  Choice(Asn1Choice),
  Integer(Asn1Integer),
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

#[derive(PartialEq, Debug)]
pub struct Asn1Choice {
  pub fields: Vec<Asn1ChoiceField>,
}

pub type Asn1SeqField = Asn1Field;
pub type Asn1ChoiceField = Asn1Field;

#[derive(PartialEq, Debug)]
pub struct Asn1Field {
  pub name: String,
  pub asn1_type: Asn1Type,
}

#[derive(PartialEq, Debug)]
pub struct Asn1Integer;
