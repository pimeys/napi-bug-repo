use std::sync::{Arc, RwLock};

use napi::{
    threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction},
    CallContext, Env, JsFunction, JsObject, JsString, JsUndefined, Property,
};
use napi_derive::{js_function, module_exports};

enum Mode {
    On(ThreadsafeFunction<String>),
    Off,
}

#[derive(Clone)]
struct Engine {
    mode: Arc<RwLock<Mode>>,
}

#[js_function(1)]
pub fn constructor(ctx: CallContext) -> napi::Result<JsUndefined> {
    let cb = ctx.get::<JsFunction>(0)?;

    let mut cb = ctx
        .env
        .create_threadsafe_function(&cb, 0, |mut ctx: ThreadSafeCallContext<String>| {
            ctx.env.adjust_external_memory(ctx.value.len() as i64)?;

            ctx.env
                .create_string_from_std(ctx.value)
                .map(|js_string| vec![js_string])
        })?;

    cb.unref(&ctx.env)?;

    let mut this: JsObject = ctx.this_unchecked();
    let mode = Mode::On(cb);
    let engine = Engine {
        mode: Arc::new(RwLock::new(mode)),
    };

    ctx.env.wrap(&mut this, engine)?;
    ctx.env.get_undefined()
}

#[js_function(1)]
fn call(ctx: CallContext) -> napi::Result<JsObject> {
    let prefix = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_string();

    let this: JsObject = ctx.this_unchecked();
    let engine: &Engine = ctx.env.unwrap(&this)?;
    let engine: Engine = engine.clone();

    let future = async move {
        for i in 1..10 {
            match &*engine.mode.read().unwrap() {
                Mode::On(cb) => {
                    cb.call(
                        Ok(format!("{}_{}", prefix, i)),
                        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
                    );
                }
                Mode::Off => (),
            }
        }

        Ok(())
    };

    ctx.env.execute_tokio_future(future, |env, ()| env.get_undefined())
}

#[js_function(0)]
fn off(ctx: CallContext) -> napi::Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let engine: &Engine = ctx.env.unwrap(&this)?;
    let engine: Engine = engine.clone();

    let future = async move {
        let mut mode = engine.mode.write().unwrap();
        *mode = Mode::Off;
        Ok(())
    };

    ctx.env.execute_tokio_future(future, |env, ()| env.get_undefined())
}

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> napi::Result<()> {
    let engine = env.define_class(
        "Engine",
        constructor,
        &[
            Property::new(&env, "call")?.with_method(call),
            Property::new(&env, "off")?.with_method(off),
        ],
    )?;

    exports.set_named_property("Engine", engine)?;

    Ok(())
}
