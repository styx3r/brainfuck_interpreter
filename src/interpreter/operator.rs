use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub trait Operator {
    fn evaluate(&self, state: &mut State);
}
