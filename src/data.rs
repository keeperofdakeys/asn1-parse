#[derive(PartialEq, Debug)]
pub enum Tagging {
  Explicit,
  Implicit
}

#[derive(PartialEq, Debug)]
pub struct Asn1Spec {
  pub tagging: Tagging,
  pub defs: Vec<Asn1Def>,
}

#[derive(PartialEq, Debug)]
pub enum Asn1Type {
  // An ordered collection of types
  Seq(Vec<Asn1Field>),
  // An ordered collection of a specific type
  SeqOf(Option<String>, Box<Asn1Type>),
  // An unordered collection of types
  Set(Vec<Asn1Field>),
  // An unordered collection of a specific type
  SetOf(Option<String>, Box<Asn1Type>),
  // A Choice
  Choice(Vec<Asn1Field>),
  // An integer
  Integer,
  // A generic Asn1Type
  Type(String),
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
pub enum Asn1Optional {
  Optional,
  Default(String),
}

#[derive(PartialEq, Debug)]
pub enum Asn1Field {
  ExtentMarker,
  Def(Asn1FieldDef),
}

#[derive(PartialEq, Debug)]
pub struct Asn1FieldDef {
  pub name: String,
  pub tag: Option<Asn1Tag>,
  pub asn1_type: Asn1Type,
  pub optional: Option<Asn1Optional>,
}
