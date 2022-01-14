use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;

mod operator;
mod increment_program_counter_operator;
mod decrement_program_counter_operator;
mod increment_cell_operator;
mod decrement_cell_operator;
mod output_cell_operator;
mod input_cell_operator;
mod jump_forward_if_zero_operator;
mod jump_backward_if_non_zero_operator;
mod nop_operator;
mod state;

use crate::interpreter::operator::Operator;
use crate::interpreter::increment_program_counter_operator::IncrementProgramCounterOperator;
use crate::interpreter::decrement_program_counter_operator::DecrementProgramCounterOperator;
use crate::interpreter::increment_cell_operator::IncrementCellOperator;
use crate::interpreter::decrement_cell_operator::DecrementCellOperator;
use crate::interpreter::input_cell_operator::InputCellOperator;
use crate::interpreter::output_cell_operator::OutputCellOperator;
use crate::interpreter::jump_backward_if_non_zero_operator::JumpBackwardIfNonZeroOperator;
use crate::interpreter::jump_forward_if_zero_operator::JumpForwardIfZeroOperator;
use crate::interpreter::nop_operator::NopOperator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct Interpreter {
    state: State
}

//------------------------------------------------------------------------------

impl Interpreter {
    fn read(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        
        // read the whole file
        file.read_to_string(&mut self.state.input)?;

        for element in self.state.input.clone().chars() {
            let operator: Box<dyn Operator> = match element {
                '>' => Box::new(IncrementProgramCounterOperator),
                '<' => Box::new(DecrementProgramCounterOperator),
                '+' => Box::new(IncrementCellOperator),
                '-' => Box::new(DecrementCellOperator),
                '.' => Box::new(OutputCellOperator),
                ',' => Box::new(InputCellOperator),
                '[' => Box::new(JumpForwardIfZeroOperator),
                ']' => Box::new(JumpBackwardIfNonZeroOperator),
                _ => Box::new(NopOperator),
            };
            
            operator.evaluate(&mut self.state);            
        }
        
        Ok(())
    }

    pub fn load_file(path: &str) -> io::Result<()> {
        let interpreter = &mut Self {
            state: State {
                program_counter: 0,
                cells: Vec::new(),
                input: String::new(),
                current_input_index: 0,
            }
        };

        interpreter.read(path)
    }
}