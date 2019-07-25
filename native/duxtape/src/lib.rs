#[macro_use]
extern crate rustler;
extern crate ducc;

// use ducc::ffi;
// use ducc::util::protect_duktape_closure;
use ducc::Ducc;
use ducc::ExecSettings;
use ducc::Value;
use rustler::resource::ResourceArc;
// use rustler::schedule::SchedulerFlags;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::mpsc;
use std::sync::Mutex;

pub struct ResponseChannel {
    sender_channel: Mutex<Option<mpsc::Sender<String>>>,
    response_channel: Mutex<Option<mpsc::Receiver<String>>>,
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

    true
}

fn compile<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let source: String = args[0].decode()?;
    assert!(source.starts_with("("));
    assert!(source.ends_with(")"));
    assert!(source.contains("function"));

    let (sender, thread_receiver) = mpsc::channel();
    let (thread_sender, receiver) = mpsc::channel();
    let resource = ResourceArc::new(ResponseChannel {
        sender_channel: Mutex::new(Some(sender)),
        response_channel: Mutex::new(Some(receiver)),
    });

    std::thread::spawn(move || {
        let ducc = Ducc::new();
        let function: Value = ducc.compile(&source, None).unwrap().call(()).unwrap();;
        ducc.globals().set("test", function).unwrap();

        // todo have branch for if message is shutdown to exit the loop
        loop {
            match thread_receiver.recv() {
                Ok(msg) => {
                    println!("got msg {:?}", msg);
                    let input = format!("test({})", msg);
                    let res = ducc.exec(&input, None, ExecSettings::default()).unwrap();
                    thread_sender.send(res).unwrap()
                }
                Err(_error) => () // this gets called when the channel hangs-up from eval ending,
            }
        }
    });

    Ok((atoms::ok(), resource).encode(env))
}

pub fn eval<'a>(env: Env<'a>, terms: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ResponseChannel> = terms[0].decode()?;
    println!("resource cant be re-used");
    let mut send_lock = resource.sender_channel.lock().unwrap();
    let mut resp_lock = resource.response_channel.lock().unwrap();

    let body: String = terms[1].decode()?;

    let sender = send_lock.take().unwrap();
    sender.send(body).unwrap();
    let res = resp_lock.take().unwrap();
    let msg_result = res.recv().unwrap();

    Ok((atoms::ok(), msg_result).encode(env))
}
