use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct NopOperator;

impl Operator for NopOperator {
    fn evaluate(&self, state: &mut State) {
        return;
    }
}