use std::any::{Any, TypeId};
use std::error::Error;
use std::collections::HashMap;

enum StepParam = (TypeId, String);
type StepValue = Box<dyn Any>;
type StepState = HashMap<StepParam, StepValue>;
type StepResult = Result<(StepValue, StepState), dyn Error>;

trait Step {
    fn run(
        &self,
        call: impl Call,
        (value, state): (StepValue, StepState),
        params: Option<Vec<StepParam>>,
        bind: Option<Vec<StepParam>>,
        // destruct option here?        
    ) -> StepResult;
}
