use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct IncrementProgramCounterOperator;

impl Operator for IncrementProgramCounterOperator {
    fn evaluate(&self, state: &mut State) {
        println!("HALLO");
    }
}