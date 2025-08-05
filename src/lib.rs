#[allow(warnings)]
mod bindings;

use std::ops::ControlFlow;

use bindings::exports::ubet::bos::pipes::Guest;

struct Component;

impl Guest for Component {
    fn pipe(steps: impl IntoIterator<Item = impl Step>, call: impl Call) -> CallResult {
        let result = steps.into_iter().try_fold(call, |this_call, step| {
            match step.take(this_call) {
                Ok(next_call) => ControlFlow::Continue(next_call),
                Err(error_call) => ControlFlow::Break(error_call),
            }
        });

        match result {
            ControlFlow::Continue(last_call) => Ok(last_call),
            ControlFlow::Break(error_call) => Err(error_call),
        }
    }
    // other top-level control structure here eventually: machine, loop, etc.
}

bindings::export!(Component with_types_in bindings);
