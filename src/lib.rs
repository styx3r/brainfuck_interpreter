mod interpreter;

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    #[test]
    fn it_works() {
        let path = "/home/styxer/GIT/brainfuck_interpreter/test_files/hello_world.bf";
        assert!(Interpreter::load_file(path).is_ok());
    }
}
