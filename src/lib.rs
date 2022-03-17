mod interpreter;

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    #[test]
    fn it_works() {
        let path = "test_files/simple_hello.bf";
        assert!(Interpreter::load_file(path).unwrap() == "Hello\n");
    }

    //-------------------------------------------------------------------------

    #[test]
    fn count_nine() {
        let path = "test_files/count_nine.bf";
        assert!(Interpreter::load_file(path).unwrap() == "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    }

    //-------------------------------------------------------------------------

    #[test]
    fn wiki_hello_world() {
        let path = "test_files/wiki_hello_world.bf";
        assert!(Interpreter::load_file(path).unwrap() == "Hello World!\n");
    }

    //-------------------------------------------------------------------------

    #[test]
    fn loop_test() {
        let path = "test_files/loop_test.bf";
        assert!(Interpreter::load_file(path).unwrap().len() == 0);
    }
}
