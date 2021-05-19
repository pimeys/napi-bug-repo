use napi::{CallContext, Env, JsObject, JsString};
use napi_derive::{js_function, module_exports};

#[js_function(1)]
fn test(ctx: CallContext) -> napi::Result<JsObject> {
    let _ = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;

    let data = serde_json::json!({
        "findFirstBooking": {
            "id": "ckovh15xa104945sj64rdk8oas",
            "name": "1883da9ff9152",
            "forename": "221c99bedc6a4",
            "description": "8bf86b62ce6a",
            "email": "9d57a869661cc",
            "phone": "7e0c58d147215",
            "arrivalDate": -92229669,
            "departureDate": 202138795,
            "price": -1592700387,
            "advance": -369294193,
            "advanceDueDate": 925000428,
            "kids": 520124290,
            "adults": 1160258464,
            "status": "NO_PAYMENT",
            "nourishment": "BB",
            "createdAt": "2021-05-19T12:58:37.246Z",
            "room": { "id": "ckovh15xa104955sj6r2tqaw1c", "name": "38683b87f2664" }
        }
    });

    ctx.env.execute_tokio_future(
        async move { Ok(serde_json::to_string(&data).unwrap()) },
        |&mut env, response| env.create_string(&response),
    )
}

#[module_exports]
pub fn init(mut exports: JsObject, _env: Env) -> napi::Result<()> {
    exports.create_named_method("test", test)?;
    Ok(())
}
