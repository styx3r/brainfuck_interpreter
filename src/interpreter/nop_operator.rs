use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct NopOperator;

impl Operator for NopOperator {
    fn evaluate(&self, _state: &mut State) {
        return;
    }
}
