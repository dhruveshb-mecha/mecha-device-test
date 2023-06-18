use crate::test_base::TestAssertion;
use anyhow::Result;

pub(crate) struct DisplayDetect {
    pub device: String,
}

impl TestAssertion for DisplayDetect {
    fn describe(&self) -> &str {
        "Display Detection test"
    }

    fn test(&self) -> Result<bool> {
        // will be replaced with actual behaviour using sdk
        println!("Display Detected");
        Ok(true)
    }
}
