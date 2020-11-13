//! This crate try to be a common interface for prolog engines,
//! For the moment only Swi Prolog engine has been integrated, but in future i would like to integrate also scryer-prolog (https://github.com/mthom/scryer-prolog).

mod data;
mod term;
mod backends;

pub use crate::term::Term;
pub use crate::data::Data;
use std::future::Future;

/**
Trait representing the interface to a prolog engine.
*/
pub trait Prolog
{
    /**
    Make a query and return it's handle. The handle is async and results can be retrieved using Swipl::block_on function.
    */
    fn query(&self, module: Option<String>,term: Term)->Box<dyn Future<Output = Result<Vec<Vec<Data>>,String>>>;
    /**
    Same as query function, but the query do not return data, only the result of the operation.
    */
    fn run(&self, module: Option<String>,term: Term)->Box<dyn Future<Output = Result<bool,String>>>;
}

/**
Function used to initialize the prolog engine. The concrete type under Prolog trait will be chosen based on crate features.
Available engines:
- Swi-Prolog: enabled using "swi_prolog_backend" feature.
*/
pub fn init()->Box<dyn Prolog>
{
    #[cfg(not(any(feature = "swi_prolog_backend")))]
    compile_error!("At least one backend must be choosen");

    //TODO Throw compile error if multiple backends are selected
/*
    #[cfg(not(any(feature = "swi_prolog_backend",feature = "caio_backend",feature = "sempronio_backend")))]
    compile_error!("At most one backend must be choosen");
*/
    #[cfg(feature = "swi_prolog_backend")]
    {
        Box::new(swi_prolog::SwiProlog::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
