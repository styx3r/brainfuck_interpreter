use log::info;

use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct DecrementCellOperator;

impl Operator for DecrementCellOperator {
    fn evaluate(&self, state: &mut State) {
        info!("Len cells: {} PC: {}", state.cells.len(), state.program_counter as usize);

        if state.program_counter + 1 > state.cells.len() as u64 {
            state.cells.resize((state.program_counter + 1) as usize, 0);
        }

        let current_cell_value = &mut state.cells[state.program_counter as usize];
        if *current_cell_value == 0x00 {
            *current_cell_value = 0xFF;
        }
        else {
            *current_cell_value -= 1;
        }

        info!("Current cell value: {}", state.cells[state.program_counter as usize]);
    }
}
