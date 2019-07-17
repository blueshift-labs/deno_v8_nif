#[macro_use]
extern crate rustler;
extern crate deno;
extern crate futures;
extern crate tokio;

use deno::*;
use futures::future::lazy;
use rustler::{Encoder, Env, NifResult, Term};
use tokio::prelude::*;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

rustler_export_nifs! {
    "Elixir.Duxtape.Native",
    [
    ("eval", 2, eval)
    ],
    None
}

fn eval<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let main_future = lazy(move || {
        let js_source = include_str!("test.js");

        let startup_data = StartupData::Script(Script {
            source: js_source,
            filename: "test.js",
        });

        let isolate = deno::Isolate::new(startup_data, false);

        isolate.then(|r| {
            // js_check(r);
            println!("zomg");
            // println!("{}", r.unwrap());
            // let res = isolate.execute("test.js", js_source.clone());
            Ok(r.unwrap())
        })
    });
    let res = tokio::run(main_future);

    Ok((atoms::ok(), res).encode(env))
}
