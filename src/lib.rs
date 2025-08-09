#[allow(warnings)]
mod bindings;

mod call;

use bindings::exports::ubet::bos::pipes::Guest;

struct Component;

impl Guest for Component {
    fn dispatch(env: call::EnvironmentMap, binargs: Vec<u8>) -> call::CallResult {
        router::dispatch(env, binargs).expect("env should determine a call").enter()
    }
}

bindings::export!(Component with_types_in bindings);
