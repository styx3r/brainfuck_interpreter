use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct IncrementCellOperator;

impl Operator for IncrementCellOperator {
    fn evaluate(&self, state: &mut State) {
        println!("HALLO");
    }
}