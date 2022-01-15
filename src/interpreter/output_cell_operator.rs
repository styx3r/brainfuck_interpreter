use log::info;

use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct OutputCellOperator;

impl Operator for OutputCellOperator {
    fn evaluate(&self, state: &mut State) {
        print!("{}", state.cells[state.program_counter as usize] as char);
    }
}