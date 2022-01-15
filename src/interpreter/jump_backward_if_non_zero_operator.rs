use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct JumpBackwardIfNonZeroOperator;

impl Operator for JumpBackwardIfNonZeroOperator {
    fn evaluate(&self, state: &mut State) {
    }
}