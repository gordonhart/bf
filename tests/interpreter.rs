/*
extern crate bf;

use bf::interpreter;

#[test]
fn simple_program() {
    let program: &'static str = "><++[-]";
    let target = interpreter::State {
        data: vec![0, 0],
        data_ptr: 0,
        program_ptr: 7,
        loop_stack: vec![],
        status: interpreter::ExecutionStatus::Terminated,
    };
    assert_eq!(target, interpreter::run(program));
}
*/
