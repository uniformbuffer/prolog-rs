/**
Representation of a term, the basic build block of prolog.
*/
#[derive(Clone,Debug)]
pub enum Term
{
    Bool(bool),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
    Variable,
    Predicate(String,Box<Vec<Term>>)
}

impl Term
{
    pub fn assert(term: Term)->Term {Term::from(("assert",vec![term]))}
    pub fn retract(term: Term)->Term {Term::from(("retract",vec![term]))}
    pub fn head_body(term1: Term,term2: Term)->Term {Term::from((":-",vec![term1,term2]))}
}

impl From<String> for Term {fn from(value: String) -> Self {Self::String(value)}}
impl From<&str> for Term {fn from(value: &str) -> Self {Self::String(value.to_string())}}
impl From<bool> for Term {fn from(value: bool) -> Self {Self::Bool(value)}}
impl From<i32> for Term {fn from(value: i32) -> Self {Self::I32(value)}}
impl From<i64> for Term {fn from(value: i64) -> Self {Self::I64(value)}}
impl From<f64> for Term {fn from(value: f64) -> Self {Self::F64(value)}}
impl From<(String,Vec<Term>)> for Term {fn from(value: (String,Vec<Term>)) -> Self {Self::Predicate(value.0,Box::new(value.1))}}
impl From<(&str,Vec<Term>)> for Term {fn from(value: (&str,Vec<Term>)) -> Self {Self::Predicate(value.0.to_string(),Box::new(value.1))}}
impl From<()> for Term {fn from(_value: ()) -> Self {Self::Variable}}
