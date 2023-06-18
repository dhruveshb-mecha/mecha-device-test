use anyhow::Result;

pub trait TestAssertion {
    fn describe(&self) -> &str;
    fn test(&self) -> Result<bool>;
}

pub struct TestRunner {
    pub suit: Vec<Box<dyn TestAssertion>>,
    pub test_count: usize,
    pub test_passed: usize,
    pub test_failed: usize,
    pub test_runner: fn(),
}

impl TestRunner {
    pub fn run(&mut self) -> Result<()> {
        for test in &self.suit {
            let result = test.test()?;
            self.test_count += 1;
            if result {
                self.test_passed += 1;
            } else {
                self.test_failed += 1;
            }
        }
        Ok(())
    }
}
