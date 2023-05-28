use crate::resource::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Selector<'a> {
  Any,
  Name(&'a str),
  AttributeEquals(&'a str, Value),
  AnyChildMatches(&'a Selector<'a>),
  And(Vec<Selector<'a>>),
  Or(Vec<Selector<'a>>),
}
