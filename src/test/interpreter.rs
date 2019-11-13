use crate::interpreter::{run, State, ExecutionStatus};

#[test]
fn simple_program() {
    let program: &'static str = "><++[-]";
    let target = State {
        data: vec![0, 0],
        data_ptr: 0,
        program_ptr: 7,
        loop_stack: vec![],
        status: ExecutionStatus::Terminated,
    };
    assert_eq!(target, run(program));
}
