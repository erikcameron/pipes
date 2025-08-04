#[allow(warnings)]
mod bindings;

use bindings::exports::ubet::bos::pipes::Guest;

struct Component;

impl Guest for Component {
    fn call(steps: steps, call: call-object) -> call-result {
    }
}

bindings::export!(Component with_types_in bindings);
