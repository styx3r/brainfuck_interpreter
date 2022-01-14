pub struct State {
    pub program_counter: i64,
    pub cells: Vec<u8>,
    pub input: String,
    pub current_input_index: u64,
}