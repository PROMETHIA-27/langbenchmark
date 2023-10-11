use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mlua::Lua;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Module, Source, Sources};

static LUASRC: &str = include_str!("../bench.lua");
static RUNESRC: &str = include_str!("../bench.rn");
static RHAISRC: &str = include_str!("../bench.rhai");
mod bench;

fn invsqrt(n: f64) -> f64 {
    n.powf(-0.5)
}

fn main() {
    // Luajit: ~1200us
    // Lua 5.4: ~2700us
    // Luau: ~2000us
    // Luau-jit: ~3000us
    // Luau-vector4: ~2000us (presumably not vectorizing automatically)
    // Rune: ~38,000us

    let lua = Lua::new();
    let print = lua
        .create_function(|_: &Lua, n: f64| Ok(println!("{n}")))
        .unwrap();
    lua.globals().set("print", print).unwrap();
    let nbody = lua.load(LUASRC);
    let len = time(|| nbody.exec().unwrap());
    println!("Lua: {}us", len.as_micros());

    let mut custom_module = Module::new();
    custom_module.function(["sqrt"], f64::sqrt).unwrap();
    custom_module.function(["invsqrt"], invsqrt).unwrap();
    let mut ctx = rune::Context::with_default_modules().unwrap();
    ctx.install(custom_module).unwrap();
    let runtime = Arc::new(ctx.runtime());
    let mut sources = Sources::new();
    sources.insert(Source::new("bench.rune", RUNESRC));
    let mut diagnostics = rune::Diagnostics::new();
    let unit = rune::prepare(&mut sources)
        .with_context(&ctx)
        .with_diagnostics(&mut diagnostics)
        .build();
    // if !diagnostics.is_empty() {
    //     let mut writer = StandardStream::stderr(ColorChoice::Always);
    //     diagnostics.emit(&mut writer, &sources).unwrap();
    // }
    let mut rune = rune::Vm::new(runtime, Arc::new(unit.unwrap()));
    let len = time(|| {
        rune.call(["main"], ())
            .map_err(|e| println!("{e}"))
            .expect("oof");
    });
    println!("Rune: {}us", len.as_micros());

    let len = time(|| {
        bench::main();
    });
    println!("Rust: {}us", len.as_micros());

    let mut engine = rhai::Engine::new();

    let len = time(|| {
        engine.register_fn("invsqrt", invsqrt);
        let () = engine.eval(RHAISRC).unwrap();
    });
    println!("Rhai: {}us", len.as_micros());

    let mut lua = piccolo::Lua::full();
    let thread = lua.run(|ctx| {
        let reader = Cursor::new(LUASRC);
        let closure = piccolo::Closure::load(ctx, reader).unwrap();
        let thread = piccolo::Thread::new(&ctx);
        thread.start(ctx, closure.into(), ()).unwrap();
        let print = piccolo::AnyCallback::from_fn(&ctx, |_, _, stack| {
            let piccolo::Value::Number(n) = stack.pop_back() else {
                panic!()
            };
            println!("{n}");
            Ok(piccolo::CallbackReturn::Return)
        });
        ctx.state.globals.set(ctx, "print", print).unwrap();
        ctx.state.registry.stash(&ctx, thread)
    });
    let len = time(|| {
        let () = lua.run_thread(&thread).unwrap();
    });
    println!("Piccolo: {}us", len.as_micros());
}

fn time(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}
