use log::info;

use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct IncrementProgramCounterOperator;

impl Operator for IncrementProgramCounterOperator {
    fn evaluate(&self, state: &mut State) {
        state.program_counter += 1;

        info!("Current PC: {}", state.program_counter);
    }
}
