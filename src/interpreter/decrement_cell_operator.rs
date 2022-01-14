use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct DecrementCellOperator;

impl Operator for DecrementCellOperator {
    fn evaluate(&self, state: &mut State) {
        println!("HALLO");
    }
}