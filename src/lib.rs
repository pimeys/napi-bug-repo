use napi::{
    threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction},
    CallContext, Env, JsFunction, JsObject, JsUndefined,
};
use napi_derive::{js_function, module_exports};

#[derive(Clone)]
struct A {
    cb: ThreadsafeFunction<String>,
}

#[js_function(1)]
fn constructor(ctx: CallContext) -> napi::Result<JsUndefined> {
    let callback = ctx.get::<JsFunction>(0)?;

    let mut cb = ctx
        .env
        .create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<String>| {
            ctx.env
                .create_string_from_std(ctx.value)
                .map(|js_string| vec![js_string])
        })?;

    cb.unref(&ctx.env)?;

    let mut this: JsObject = ctx.this_unchecked();
    let obj = A { cb };

    ctx.env.wrap(&mut this, obj)?;
    ctx.env.get_undefined()
}

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> napi::Result<()> {
    let obj = env.define_class("A", constructor, &[])?;

    exports.set_named_property("A", obj)?;

    Ok(())
}
