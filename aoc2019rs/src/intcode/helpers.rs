use super::{IntcodeMachine, IntcodeOutput, IntcodeInstruction};

pub fn process_input(program: &[i64], inputs: &[i64]) -> Vec<String> {
    let mut machine = IntcodeMachine::new_automated_machine(program, inputs);
    machine.run();
    let (_, _, _, output_handler) = machine.teardown();
    output_handler.history().to_vec()
}

pub fn debug_process_input(program: &[i64], inputs: &[i64]) -> (Vec<IntcodeInstruction>, Vec<String>) {
    let mut machine = IntcodeMachine::new_automated_machine(program, inputs);
    let instructions = machine.debug();
    let (_, _, _, output_handler) = machine.teardown();
    (instructions, output_handler.history().to_vec())
}
