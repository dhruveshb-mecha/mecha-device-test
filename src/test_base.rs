use std::io::{self, Write};

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

pub fn question_prompt(question: String) -> bool {
    print!("{} [y/n] ", question);
    let mut answer = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    match answer {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => {
            println!("Invalid input, please try again");
            question_prompt(question)
        }
    }
}
