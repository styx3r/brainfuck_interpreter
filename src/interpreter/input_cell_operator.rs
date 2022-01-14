use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct InputCellOperator;

impl Operator for InputCellOperator {
    fn evaluate(&self, state: &mut State) {
        println!("HALLO");
    }
}