use log::info;

use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct IncrementCellOperator;

impl Operator for IncrementCellOperator {
    fn evaluate(&self, state: &mut State) {
        info!("Len cells: {} PC: {}", state.cells.len(), state.program_counter as usize);
        if state.cells.len() <= state.program_counter as usize {
            state.cells.push(1);
        }
        else {
            let current_cell_value = &mut state.cells[state.program_counter as usize];
            if *current_cell_value == 255 {
                *current_cell_value = 0;
            }
            else {
                *current_cell_value += 1;
            }
        }

        info!("Current cell value: {}", state.cells[state.program_counter as usize]);
    }
}