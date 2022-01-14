use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct DecrementProgramCounterOperator;

impl Operator for DecrementProgramCounterOperator {
    fn evaluate(&self, state: &mut State) {
        println!("HALLO");
    }
}