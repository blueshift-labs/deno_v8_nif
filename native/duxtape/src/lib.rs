#[macro_use]
extern crate rustler;

extern crate ducc;

use ducc::Ducc;
use ducc::ExecSettings;
use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler_export_nifs! {
    "Elixir.Duxtape.Native",
    [
    ("eval", 1, eval)
    ],
    None
}

fn eval<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let source: String = args[0].decode()?;
    let ducc = Ducc::new();

    let result: Result<String, _> = ducc.exec(&source, None, ExecSettings::default());
    match result {
        Ok(res) => Ok((atoms::ok(), res).encode(env)),
        Err(error) => Ok((atoms::error(), format!("{}", error)).encode(env)),
    }
}
