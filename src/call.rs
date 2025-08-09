use std::any::Any;
use std::error::Error;
use std::collections::HashMap;
use std::ops::ControlFlow;
use uuid::Uuid;
use wasi::clocks::monotonic_clock;
use wasi::clocks::monotonic_clock::Instant;

mod step;

pub type Id = Uuid;
pub type EnvironmentMap = HashMap<String, String>;
pub type CallResult = Result<dyn Call, dyn Error>;

struct Meta<T> {
    id: Id,
    env: EnvironmentMap,
    binargs: Vec<u8>,
    start_at: Option<Instant>,
    end_at: Option<Instant>,
    ok: Option<(StepValue, StepState)>,
    error: Option<dyn Error>,
}

trait Call: Iterator<Item = dyn Step> + Step {
    // pretty sure there's a more efficient way to do this with &str

    // a constructor that takes an environment and a neutral argument format
    fn new(env: EnvironmentMap, binargs: Vec<u8>) -> Self;

    // call types must implement this method to provide a Meta struct
    fn meta(&self) -> Meta;

    //
    // default implementations
    //

    // eventually structured logging goes here
    // fn log<T>(&self, level: Level, payload: T) {
    // }

    // getters for read only access within other components
    fn id(&self) -> Id { &self.id }
    fn env(&self) -> EnvironmentMap { &self.env }
    fn binargs(&self) -> Vec<u8> { &self.binargs }
    fn ok(&self) -> Option<(StepValue, StepState)> { &self.ok }
    fn error(&self) -> Option<impl Error> { &self.error }

    // implements Step so we can run Calls
    fn run(&self, call: impl Call, (value, state): (StepValue, StepState)) -> StepResult {
        self.meta.start_at = monotonic_clock::now();

        let result = self.try_fold((value, state), |(current_value, current_state), step| {
            match step.run(call, (current_value, current_state)) {
                Ok((next_value, next_state)) => {
                    self.set_ok((next_value, next_state));
                    ControlFlow::Continue((next_value, next_state))
                },
                Err(error) => {
                    self.set_error(error);
                    ControlFlow::Break(error)
                },
            }
        });

        self.meta.end_at = monotonic_clock::now();

        match result {
            ControlFlow::Continue((value, state)) => Ok((self, state)),
            ControlFlow::Break(error) => Err(error),
        }
    }

    fn set_ok(&self, ok: (StepValue, StepState)) { self.call_meta.ok = ok; }
    fn set_error(&self, error: impl Error) { self.call_meta.error = error; }
}
