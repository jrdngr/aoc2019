pub trait IntcodeOutput {
    fn process(&mut self, value: i64);
    fn history(&self) -> &[String];
    fn last_output(&self) -> Option<&String> {
        self.history().last()
    }
}

pub struct IntcodeConsoleOutput {
    history: Vec<String>,
}

impl IntcodeConsoleOutput {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }
}

impl IntcodeOutput for IntcodeConsoleOutput {
    fn process(&mut self, value: i64) {
        println!("Output: {}", value);
    }

    fn history(&self) -> &[String] {
        &self.history
    }
}

pub struct IntcodeHistoryOutput {
    history: Vec<String>,
}

impl IntcodeHistoryOutput {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }
}
impl IntcodeOutput for IntcodeHistoryOutput {
    fn process(&mut self, value: i64) {
        self.history.push(format!("{}", value));
    }
    
    fn history(&self) -> &[String] {
        &self.history
    }
}
