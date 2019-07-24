#[macro_use]
extern crate rustler;
extern crate ducc;

// use ducc::ffi;
// use ducc::util::protect_duktape_closure;
use ducc::Ducc;
use ducc::{Value, Values};
// use ducc::ExecSettings;
use rustler::resource::ResourceArc;
// use rustler::schedule::SchedulerFlags;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::mpsc;
use std::sync::Mutex;

pub struct ResponseChannel {
    sender_channel: Mutex<Option<mpsc::Sender<String>>>,
    response_channel: Mutex<Option<mpsc::Receiver<String>>>,
}

pub struct ShutdownChannel {
    shutdown_channel: Mutex<Option<mpsc::Sender<String>>>,
}

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler_export_nifs! {
    "Elixir.Duxtape.Native",
    [
    ("compile", 1, compile),
    ("eval", 2, eval),
    // ("run_func", 1, run_func, SchedulerFlags::DirtyCpu)
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _: Term<'a>) -> bool {
    rustler::resource_struct_init!(ResponseChannel, env);
    rustler::resource_struct_init!(ShutdownChannel, env);

    true
}

fn compile<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let source: String = args[0].decode()?;

    let (sender, thread_receiver) = mpsc::channel();
    let (thread_sender, receiver) = mpsc::channel();
    let resource = ResourceArc::new(ResponseChannel {
        sender_channel: Mutex::new(Some(sender)),
        response_channel: Mutex::new(Some(receiver)),
    });

    std::thread::spawn(move || {
        let ducc = Ducc::new();
        let function = ducc.compile(&source, None).unwrap();

        // todo have branch for if message is shutdown to exit the loop
        loop {
            let msg = thread_receiver.recv().unwrap();
            println!("got msg {:?}", msg);
            // below is needed so we can keep calling function because an invocation consumes it from the stack
            // protect_duktape_closure(ducc.ctx, 0, 2, |ctx| ffi::duk_dup_top(ctx));

            // let string = ducc.create_string("test").unwrap();
            let duc_string = ducc.create_string(&msg).unwrap();
            let string = Value::String(duc_string);
            let vec = vec![string];
            // let string = Vec::new(msg);
            let result: Result<String, _> = function.call(Values::from_vec(vec));
            thread_sender.send(result.unwrap());
        }
    });

    Ok((atoms::ok(), resource).encode(env))
}

pub fn eval<'a>(env: Env<'a>, terms: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ResponseChannel> = terms[0].decode()?;
    let mut send_lock = resource.sender_channel.lock().unwrap();
    let mut resp_lock = resource.response_channel.lock().unwrap();

    let body: String = terms[1].decode()?;

    let sender = send_lock.take().unwrap();
    sender.send(body).unwrap();
    let res = resp_lock.take().unwrap();
    let msg_result = res.recv().unwrap();
    println!("received a {:?}", msg_result);

    Ok((atoms::ok(), msg_result).encode(env))
}
