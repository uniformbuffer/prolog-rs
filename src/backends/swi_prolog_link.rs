extern crate swi_prolog;

use crate::*;

use swi_prolog::SwiProlog;

impl From<swi_prolog::Data> for Data
{
    fn from(data: swi_prolog::Data) -> Self {
        match data
        {
            swi_prolog::Data::String(value)=>Data::String(value),
            swi_prolog::Data::Bool(value)=>Data::Bool(value),
            swi_prolog::Data::U32(value)=>Data::U32(value),
            swi_prolog::Data::U64(value)=>Data::U64(value),
            swi_prolog::Data::I32(value)=>Data::I32(value),
            swi_prolog::Data::I64(value)=>Data::I64(value),
            swi_prolog::Data::F32(value)=>Data::F32(value),
            swi_prolog::Data::F64(value)=>Data::F64(value),
        }
    }
}
impl From<Data> for swi_prolog::Data
{
    fn from(data: Data) -> Self {
        match data
        {
            Data::String(value)=>swi_prolog::Data::String(value),
            Data::Bool(value)=>swi_prolog::Data::Bool(value),
            Data::U32(value)=>swi_prolog::Data::U32(value),
            Data::U64(value)=>swi_prolog::Data::U64(value),
            Data::I32(value)=>swi_prolog::Data::I32(value),
            Data::I64(value)=>swi_prolog::Data::I64(value),
            Data::F32(value)=>swi_prolog::Data::F32(value),
            Data::F64(value)=>swi_prolog::Data::F64(value),
        }
    }
}

impl From<swi_prolog::Term> for Term
{
    fn from(data: swi_prolog::Term) -> Self {
        match data
        {
            swi_prolog::Term::String(value)=>Term::String(value),
            swi_prolog::Term::Bool(value)=>Term::Bool(value),
            swi_prolog::Term::I32(value)=>Term::I32(value),
            swi_prolog::Term::I64(value)=>Term::I64(value),
            swi_prolog::Term::F64(value)=>Term::F64(value),
            swi_prolog::Term::Variable=>Term::Variable,
            swi_prolog::Term::Predicate(name,terms)=>
            {
                let converted_terms: Box<Vec<Term>> = Box::new(terms.into_iter().map(|term|{
                    let converted_term: Term = term.into();
                    converted_term
                }).collect());
                Term::Predicate(name,converted_terms)
            }
        }
    }
}
impl From<Term> for swi_prolog::Term
{
    fn from(data: Term) -> Self {
        match data
        {
            Term::String(value)=>swi_prolog::Term::String(value),
            Term::Bool(value)=>swi_prolog::Term::Bool(value),
            Term::I32(value)=>swi_prolog::Term::I32(value),
            Term::I64(value)=>swi_prolog::Term::I64(value),
            Term::F64(value)=>swi_prolog::Term::F64(value),
            Term::Variable=>swi_prolog::Term::Variable,
            Term::Predicate(name,terms)=>
            {
                let converted_terms: Box<Vec<swi_prolog::Term>> = Box::new(terms.into_iter().map(|term|term.into()).collect());
                swi_prolog::Term::Predicate(name,converted_terms)
            }
        }
    }
}

impl Prolog for SwiProlog
{
    /**
    Make a query and return it's handle. The handle is async and results can be retrieved using Swipl::block_on function.
    */
    fn query(&self, module_name: Option<String>,term: Term)->Box<dyn Future<Output = Result<Vec<Vec<Data>>,String>>>
    {
        let self_clone = self.clone();
        let future = async move{
            match SwiProlog::query(&self_clone,module_name,term.into()).await
            {
                Ok(Ok(results))=>Ok(results.into_iter().map(|data_vec|data_vec.into_iter().map(|data|data.into()).collect()).collect()),
                Ok(Err(error_string))=>Err(error_string),
                Err(join_error)=>Err(join_error.to_string())
            }
        };
        Box::new(future)
    }

    /**
    Same as query function, but the query do not return data, only the result of the operation.
    */
    fn run(&self, module_name: Option<String>,term: Term)->Box<dyn Future<Output = Result<bool,String>>>
    {
        let self_clone = self.clone();
        let future = async move{
            SwiProlog::run(&self_clone,module_name,term.into()).await.unwrap()
        };
        Box::new(future)
    }
}
