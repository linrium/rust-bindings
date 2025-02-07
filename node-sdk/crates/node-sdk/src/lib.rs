use neon::prelude::*;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize, Serialize)]
struct Ip {
    origin: String,
}

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

async fn fetch_data() -> Result<Ip, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;

    Ok(resp)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let result = fetch_data().await;
        deferred.settle_with(&channel, move |mut cx| {
            let ip = result.or_else(|err| cx.throw_error(err.to_string()))?;
            let obj: Handle<JsObject> = cx.empty_object();
            let origin = cx.string(ip.origin);
            obj.set(&mut cx, "origin", origin)?;



            Ok(obj)
        })
    });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
