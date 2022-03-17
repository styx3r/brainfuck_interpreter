pub struct State {
    pub program_counter: u64,
    pub cells: Vec<u8>,
    pub input: String,
    current_input_index: u64,
    output: String,
}


impl State {
    pub(crate) fn new() -> State {
        State {
            program_counter: 0,
            cells: vec![0],
            input: String::new(),
            current_input_index: 0,
            output: String::new(),
        }
    }

    pub(crate) fn get_current_input_index(&self) -> u64 {
        return self.current_input_index;
    }

    pub(crate) fn increment_current_input_index(&mut self) {
        self.current_input_index += 1;
    }

    pub(crate) fn decrement_current_input_index(&mut self) {
        self.current_input_index -= 1;
    }

    pub(crate) fn set_current_input_index(&mut self, index: u64) {
        self.current_input_index = index;
    }

    pub(crate) fn append_to_output(&mut self, chr: char) {
        self.output.push(chr);
    }

    pub(crate) fn get_output(&self) -> String {
        return self.output.clone();
    }
}
