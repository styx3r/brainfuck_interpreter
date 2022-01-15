use log::info;

use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct DecrementCellOperator;

impl Operator for DecrementCellOperator {
    fn evaluate(&self, state: &mut State) {
        if state.cells.is_empty() {
            state.cells.push(255);
        }
        else {
            let current_cell_value = &mut state.cells[state.program_counter as usize];
            if *current_cell_value == 0 {
                *current_cell_value = 255;
            }
            else {
                *current_cell_value -= 1;
            }
        }

        info!("Current cell value: {}", state.cells[state.program_counter as usize]);
    }
}