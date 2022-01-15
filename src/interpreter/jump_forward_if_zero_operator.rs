use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct JumpForwardIfZeroOperator;

impl Operator for JumpForwardIfZeroOperator {
    fn evaluate(&self, state: &mut State) {
    }
}