use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
) -> jstring {
    // let input: String = env
    //     .get_string(&input)
    //     .expect("couldn't get java string")
    //     .into();

    let output = env
        .new_string(format!("Hello, {}", "linh"))
        .expect("couldn't get java string");

    output.into_raw()
}
