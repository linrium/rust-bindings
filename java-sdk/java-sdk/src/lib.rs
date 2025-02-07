use jni::objects::{JClass, JObject};
use jni::JNIEnv;

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize, Serialize)]
struct Ip {
    origin: String,
}

async fn fetch_data() -> Result<Ip, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;

    Ok(resp)
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_asyncComputation(
    mut env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
    // functions in other threads, we must first obtain the `JavaVM` interface
    // which, unlike `JNIEnv` is `Send`.
    let jvm = env.get_java_vm().unwrap();

    // We need to obtain global reference to the `callback` object before sending
    // it to the thread, to prevent it from being collected by the GC.
    let callback = env.new_global_ref(callback).unwrap();

    let rt = Runtime::new().unwrap();

    let class = env.find_class("Ip").expect("Couldn't find Ip class!");
    let constructor_sig = "(Ljava/lang/String;)V";

    rt.block_on(async {
        let mut env = jvm.attach_current_thread().unwrap();
        let result = fetch_data().await.unwrap();
        let origin = env
            .new_string(result.origin)
            .expect("Couldn't create java string!");

        let obj = env.new_object(class, constructor_sig, &[(&origin).into()]).unwrap();

        env.call_method(
            &callback,
            "asyncCallback",
            "(LIp;)V",
            &[(&obj).into()],
        )
        .unwrap();
    });
}
