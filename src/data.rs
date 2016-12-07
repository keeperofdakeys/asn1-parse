#[derive(PartialEq, Debug)]
pub struct Asn1Spec {
  pub defs: Vec<Asn1Def>,
}

#[derive(PartialEq, Debug)]
pub enum Asn1Type {
  Type(String),
  Seq(Asn1Seq),
  Set(Asn1Set),
  Choice(Asn1Choice),
  Integer(Asn1Integer),
}

#[derive(PartialEq, Debug)]
pub enum Asn1Class {
  /// Universal class.
  Universal,
  /// Application class.
  Application,
  /// Context-specific class.
  ContextSpecific,
  /// Private class.
  Private,
}

impl Asn1Class {
  pub fn parse(input: Option<String>) -> Result<Self, ()> {
    match input {
      None => Ok(Asn1Class::ContextSpecific),
      Some(ref i) => match &*i.to_lowercase() {
        "application" => Ok(Asn1Class::Application),
        "private" => Ok(Asn1Class::Private),
        _ => Err(()),
      },
    }
  }
}

pub type Asn1Tag = (Asn1Class, u64);

#[derive(PartialEq, Debug)]
pub struct Asn1Def {
  pub name: String,
  pub tag: Option<Asn1Tag>,
  pub assign: Asn1Type,
}

#[derive(PartialEq, Debug)]
pub struct Asn1Seq {
  pub fields: Vec<Asn1SeqField>,
}

pub type Asn1Set = Asn1Seq;
pub type Asn1Choice = Asn1Seq;

pub type Asn1SeqField = Asn1Field;
pub type Asn1SetField = Asn1Field;
pub type Asn1ChoiceField = Asn1Field;

#[derive(PartialEq, Debug)]
pub enum Asn1Optional {
  Optional,
  Default(String),
}

#[derive(PartialEq, Debug)]
pub struct Asn1Field {
  pub name: String,
  pub tag: Option<Asn1Tag>,
  pub asn1_type: Asn1Type,
  pub optional: Option<Asn1Optional>,
}

#[derive(PartialEq, Debug)]
pub struct Asn1Integer;
