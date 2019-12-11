use super::{IntcodeMachine, IntcodeOutput};

pub fn process_input(program: &[i64], inputs: &[i64]) -> Vec<String> {
    let mut machine = IntcodeMachine::new_automated_machine(program, inputs);
    machine.run();
    let (_, _, _, output_handler) = machine.teardown();
    output_handler.history().to_vec()
}

pub fn process_input_last_output(program: &[i64], inputs: &[i64]) -> Option<String> {
    process_input(program, inputs).last().cloned()
}
