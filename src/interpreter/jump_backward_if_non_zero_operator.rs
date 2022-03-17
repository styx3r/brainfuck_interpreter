use crate::interpreter::operator::Operator;
use crate::interpreter::state::State;

//------------------------------------------------------------------------------

pub struct JumpBackwardIfNonZeroOperator;

impl Operator for JumpBackwardIfNonZeroOperator {
    fn evaluate(&self, state: &mut State) {
        if state.cells[state.program_counter as usize] == 0 {
            return;
        }

        let mut brackets_to_skip = 0;
        for index in (0..(state.get_current_input_index() - 1) as usize).rev() {
            match state.input.as_bytes()[index] {
                b']' => {
                    brackets_to_skip += 1;
                },
                b'[' => {
                    if brackets_to_skip == 0 {
                        state.set_current_input_index(index as u64);
                        break;
                    }
                    else {
                        brackets_to_skip -= 1;
                    }
                },
                _ => {}
            }
        }
    }
}
